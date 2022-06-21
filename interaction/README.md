## Running

**[Serde Crate](https://crates.io/crates/serde)**

>*Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.

>*The Serde ecosystem consists of data structures that know how to serialize and deserialize themselves along with data formats that know how to serialize and deserialize other things. Serde provides the layer by which these two groups interact with each other, allowing any supported data structure to be serialized and deserialized using any supported data format.*
>


![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)


Sending a POST request URL with a JSON body, we get the value as return.
![First](https://i.imgur.com/8JQYFon.png)


Request a shuffle of value "Guilherme" encoded in Base64, the return is 3542971860 decoded from Base64.
![Second](https://i.imgur.com/DgWZvFX.png)


Request random color, using both String "black" and hex value.
![Third](https://i.imgur.com/Ix3CsgD.png)


Request unknow value.

![Fourth](https://i.imgur.com/YjpjnVk.png)



## Checking different formats

First we check with JSON Format, then CBOR and then an unsupported format.
![Formats](https://i.imgur.com/d1tZ1RR.png)


