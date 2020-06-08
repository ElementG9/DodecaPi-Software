# DodecaPi Protocol Version 1

# Summary

The parent connects to each child and sends these packets:

P->C - 0x00 Handshake

C->P - 0x08 Handshake Response

Todo: P->C - 0x01 Encryption Request

Todo: C->P - 0x02 Encryption Response

Todo: P->C - 0x03 Compression Request

Todo: C->P - 0x04 Compression Response

P->C - 0x05 Factor Request

C->P - 0x06 Factor Response

P->C - 0x07 Disconnect

If either the parent or the child receives a malformed packet, they should send
a 0x07 Disconnect to end the connection.

# Packet Format

## Without Compression

| Field Name | Field Type | Notes                        |
|------------|------------|------------------------------|
| Length     | u32        | Length of Packet ID + Data   |
| Packet ID  | u8         |                              |
| Data       | byte array | Contents depend on Packet ID |

## With Compression

| Field Name            | Field Type                 | Notes                        |
|-----------------------|----------------------------|------------------------------|
| Length                | u32                        | Length of Packet ID + Data   |
| Packet ID             | u8                         |                              |
| Compressed Data       | gzip compressed byte array | Contents depend on Packet ID |

# Data Types

| Data Type  | Size (bytes) | Encodes                              | Notes                                             |
|------------|--------------|--------------------------------------|---------------------------------------------------|
| boolean    | 1            | A true or false value.               | True = 0x01 and False = 0x00                      |
| u8         | 1            | Unsigned 8-bit integer.              | Also called a byte.                               |
| u16        | 2            | Unsigned 16-bit integer.             | Also called a short.                              |
| u32        | 4            | Unsigned 32-bit integer.             | Also called an int.                               |
| u64        | 8            | Unsigned 64-bit integer.             | Also called a long.                               |
| string     | unknown      | A valid UTF-8 encoded string.        | u32 sent with length first, then the string data. |
| byte array | unknown      | Any valid sequence of bytes.         | Contents determined by packet id.                 |

# Parent-bound Packets

## Todo: 0x02 Encryption Response

## Todo: 0x04 Compression Response

## 0x06 Factor Response

| Field Name        | Field Type    | Notes                                                                                    |
|-------------------|---------------|------------------------------------------------------------------------------------------|
| Found Factor      | boolean       | Only true if the child found a factor. If false, the other fields should be ignored.     |
| Factor Value Type | string        | Either "u8", "u16", "u32", or "u64". Defaults to "u8" if Found Factor is false.          |
| Factor Value      | predetermined | The type was determined in Factor Value Type. Defaults to 0x00 if Found Factor is false. |

0x06 Factor Response returns the results of the child's work for factoring the
range provided. If no factor was found in the range, then Found Factor is false.
If Found Factor is false, Factor Value Type defaults to "u8" and Factor Value to
0x00. If Found Factor is true, Factor Value type is the type provided in
0x05 Factor Request, and Factor Value contains the factor.

## 0x08 Handshake Response

| Field Name | Field Type | Notes |
|------------|------------|-------|
|            |            |       |

Ox08 Handshake Response should leave the data field empty. Should only be sent if
the protocol version received in 0x00 Handshake is compatible with the child
version. If the protocols are incompatible, the child should send 0x07 Disconnect
instead.

# Child-bound Packets

## 0x00 Handshake

| Field Name      | Field Type | Notes |
|-----------------|------------|-------|
| Protocol Number | u8         |       |

Ox00 Handshake should only send a u8 for the protocol number. If the child does
not support the protocol version, they should send a 0x07 Disconnect in response
instead of 0x08 Handshake Response.

## Todo: 0x01 Encryption Request

## Todo: 0x03 Compression Request

## 0x05 Factor Request

| Field Name  | Field Type    | Notes                                  |
|-------------|---------------|----------------------------------------|
| Range Type  | string        | Either "u8", "u16", "u32", or "u64".   |
| Range Start | predetermined | The type was determined in Range Type. |
| Range End   | predetermined | The type was determined in Range Type. |

0x05 Factor Request should send either "u8", "u16", "u32", or "u64" to determine
the type of the range start and end. Then, the range start should be sent as the
predetermined type. Then, the range end should be sent as the predetermined type.

# Packets for Either

## 0x07 Disconnect

| Field Name | Field Type | Notes |
|------------|------------|-------|
|            |            |       |

0x07 Disconnect leaves the data field empty, but the sender and recipient
should both attempt to close the connection after sending / receiving it.
