import * as BufferLayout from '@solana/buffer-layout'
import { PublicKey } from '@solana/web3.js'
import BN from 'bn.js'
import { blob, Layout as LayoutCls, offset, seq, struct, u8, u32 } from 'buffer-layout'

export interface Layout<T> {
  span: number
  property?: string

  decode: (b: Buffer, offset?: number) => T

  encode: (src: T, b: Buffer, offset?: number) => number

  getSpan: (b: Buffer, offset?: number) => number

  replicate: (name: string) => this
}

/**
 * BN-based layout for data decoding using buffer-layout.
 * Used for decoding on-chain account data (StakePool, ValidatorList, etc.)
 */
class BNDataLayout extends LayoutCls<BN> {
  blobLayout: Layout<Buffer>
  signed: boolean

  constructor(span: number, signed: boolean, property?: string) {
    super(span, property)
    this.blobLayout = blob(span)
    this.signed = signed
  }

  decode(b: Buffer, offset = 0) {
    const num = new BN(this.blobLayout.decode(b, offset), 10, 'le')
    if (this.signed) {
      return num.fromTwos(this.span * 8).clone()
    }
    return num
  }

  encode(src: BN, b: Buffer, offset = 0) {
    if (this.signed) {
      src = src.toTwos(this.span * 8)
    }
    return this.blobLayout.encode(src.toArrayLike(Buffer, 'le', this.span), b, offset)
  }
}

/**
 * Creates a u64 layout for data decoding (account layouts).
 * Used in StakePoolLayout, ValidatorListLayout, etc.
 */
export function u64(property?: string): Layout<BN> {
  return new BNDataLayout(8, false, property)
}

/**
 * BN-based layout for 64-bit unsigned integers using @solana/buffer-layout.
 * Used for encoding instruction data with support for values > MAX_SAFE_INTEGER.
 */
class BNInstructionLayout extends BufferLayout.Layout<BN> {
  blobLayout: BufferLayout.Blob
  signed: boolean

  constructor(span: number, signed: boolean, property?: string) {
    super(span, property)
    this.blobLayout = BufferLayout.blob(span)
    this.signed = signed
  }

  decode(b: Uint8Array, offset = 0): BN {
    const num = new BN(this.blobLayout.decode(b, offset), 10, 'le')
    if (this.signed) {
      return num.fromTwos(this.span * 8).clone()
    }
    return num
  }

  encode(src: BN, b: Uint8Array, offset = 0): number {
    if (this.signed) {
      src = src.toTwos(this.span * 8)
    }
    return this.blobLayout.encode(src.toArrayLike(Buffer, 'le', this.span), b, offset)
  }

  getSpan(_b?: Uint8Array, _offset?: number): number {
    return this.span
  }
}

/**
 * Creates a u64 layout for instruction encoding.
 * Properly handles BN values larger than Number.MAX_SAFE_INTEGER.
 * Compatible with @solana/buffer-layout.struct().
 */

export function u64Instruction(property?: string): any {
  return new BNInstructionLayout(8, false, property)
}

class WrappedLayout<T, U> extends LayoutCls<U> {
  layout: Layout<T>
  decoder: (data: T) => U
  encoder: (src: U) => T

  constructor(
    layout: Layout<T>,
    decoder: (data: T) => U,
    encoder: (src: U) => T,
    property?: string,
  ) {
    super(layout.span, property)
    this.layout = layout
    this.decoder = decoder
    this.encoder = encoder
  }

  decode(b: Buffer, offset?: number): U {
    return this.decoder(this.layout.decode(b, offset))
  }

  encode(src: U, b: Buffer, offset?: number): number {
    return this.layout.encode(this.encoder(src), b, offset)
  }

  getSpan(b: Buffer, offset?: number): number {
    return this.layout.getSpan(b, offset)
  }
}

export function publicKey(property?: string): Layout<PublicKey> {
  return new WrappedLayout(
    blob(32),
    (b: Buffer) => new PublicKey(b),
    (key: PublicKey) => key.toBuffer(),
    property,
  )
}

class OptionLayout<T> extends LayoutCls<T | null> {
  layout: Layout<T>
  discriminator: Layout<number>

  constructor(layout: Layout<T>, property?: string) {
    super(-1, property)
    this.layout = layout
    this.discriminator = u8()
  }

  encode(src: T | null, b: Buffer, offset = 0): number {
    if (src === null || src === undefined) {
      return this.discriminator.encode(0, b, offset)
    }
    this.discriminator.encode(1, b, offset)
    return this.layout.encode(src, b, offset + 1) + 1
  }

  decode(b: Buffer, offset = 0): T | null {
    const discriminator = this.discriminator.decode(b, offset)
    if (discriminator === 0) {
      return null
    } else if (discriminator === 1) {
      return this.layout.decode(b, offset + 1)
    }
    throw new Error(`Invalid option ${this.property}`)
  }

  getSpan(b: Buffer, offset = 0): number {
    const discriminator = this.discriminator.decode(b, offset)
    if (discriminator === 0) {
      return 1
    } else if (discriminator === 1) {
      return this.layout.getSpan(b, offset + 1) + 1
    }
    throw new Error(`Invalid option ${this.property}`)
  }
}

export function option<T>(layout: Layout<T>, property?: string): Layout<T | null> {
  return new OptionLayout<T>(layout, property)
}

export function bool(property?: string): Layout<boolean> {
  return new WrappedLayout(u8(), decodeBool, encodeBool, property)
}

function decodeBool(value: number): boolean {
  if (value === 0) {
    return false
  } else if (value === 1) {
    return true
  }
  throw new Error(`Invalid bool: ${value}`)
}

function encodeBool(value: boolean): number {
  return value ? 1 : 0
}

export function vec<T>(elementLayout: Layout<T>, property?: string): Layout<T[]> {
  const length = u32('length')
  const layout: Layout<{ values: T[] }> = struct([
    length,
    seq(elementLayout, offset(length, -length.span), 'values'),
  ])
  return new WrappedLayout(
    layout,
    ({ values }) => values,
    values => ({ values }),
    property,
  )
}
