# DodecaPi Protocol Version 3

# Summary

The master connects to each worker and sends these packets:

M->W - [0x00 Handshake](#0x00-handshake)

W->M - [0x08 Handshake Response](#0x08-handshake-response)

Todo: M->W - [0x01 Encryption Request](#0x01-encryption-request)

Todo: W->M - [0x02 Encryption Response](#0x02-encryption-response)

Todo: M->W - [0x03 Compression Request](#0x03-compression-request)

Todo: W->M - [0x04 Compression Response](#0x04-compression-response)

M->W - [0x05 Factor Request](#0x05-factor-request)

W->M - [0x06 Factor Response](#0x06-factor-response)

M->W - [0x07 Disconnect](#0x07-disconnect)

If either the master or the worker receives a malformed packet, they should send
a [0x07 Disconnect](#0x07-disconnect) to end the connection.

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

# Master-bound Packets

## Todo: 0x02 Encryption Response

## Todo: 0x04 Compression Response

## 0x06 Factor Response

| Field Name        | Field Type    | Notes                                                                                    |
|-------------------|---------------|------------------------------------------------------------------------------------------|
| Found Factor      | boolean       | Only true if the worker found a factor. If false, the other fields should be ignored.     |
| Factor Value Type | string        | Either "u8", "u16", "u32", or "u64". Defaults to "u8" if Found Factor is false.          |
| Factor Value      | predetermined | The type was determined in Factor Value Type. Defaults to 0x00 if Found Factor is false. |

[0x06 Factor Response](#0x06-factor-response) returns the results of the worker's work for factoring the
range provided. If no factor was found in the range, then Found Factor is false.
If Found Factor is false, Factor Value Type defaults to "u8" and Factor Value to
0x00. If Found Factor is true, Factor Value type is the type provided in
[0x05 Factor Request](#0x05-factor-request), and Factor Value contains the factor.

## 0x08 Handshake Response

| Field Name | Field Type | Notes |
|------------|------------|-------|
|            |            |       |

Ox08 Handshake Response should leave the data field empty. Should only be sent if
the protocol version received in [0x00 Handshake](#0x00-handshake) is compatible with the worker
version. If the protocols are incompatible, the worker should send [0x07 Disconnect](#0x07-disconnect)
instead.

## 0x10 Pong

| Field Name | Field Type | Notes |
|------------|------------|-------|
|            |            |       |

[0x10 Pong](#0x10-pong) leaves the data field empty.

# Worker-bound Packets

## 0x00 Handshake

| Field Name      | Field Type | Notes |
|-----------------|------------|-------|
| Protocol Number | u8         |       |
| Next State      | u8         |       |

0x00 Handshake should send the master's protocol version, and the next state. Next
State can either be 0x00 for Ping, or 0x01 for Work. If the worker does not
support the protocol version, they should send a [0x07 Disconnect](#0x07-disconnect)
in response instead of [0x08 Handshake Response](#0x08-handshake-response).

## Todo: 0x01 Encryption Request

## Todo: 0x03 Compression Request

## 0x05 Factor Request

| Field Name  | Field Type    | Notes                                  |
|-------------|---------------|----------------------------------------|
| Range Type  | string        | Either "u8", "u16", "u32", or "u64".   |
| Range Start | predetermined | The type was determined in Range Type. |
| Range End   | predetermined | The type was determined in Range Type. |

[0x05 Factor Request](#0x05-factor-request) should send either "u8", "u16", "u32", or "u64" to determine
the type of the range start and end. Then, the range start should be sent as the
predetermined type. Then, the range end should be sent as the predetermined type.

## 0x09 Ping

| Field Name | Field Type | Notes |
|------------|------------|-------|
|            |            |       |

[0x09 Ping](#0x09-ping) leaves the data field empty, and the worker should respond with a
[0x10 Pong](#0x10-pong).

# Packets for Either

## 0x07 Disconnect

| Field Name | Field Type | Notes |
|------------|------------|-------|
|            |            |       |

[0x07 Disconnect](#0x07-disconnect) leaves the data field empty, but the sender and recipient should both attempt to close the connection after sending / receiving it.
