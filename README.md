# RIPCALC

RIPCALC is an IP Calculator made with Rust that shows information and allows you to subnet an IP Address.

## Compile

To compile just run the following command.

```bash
cargo build --release
```

You will find the executable on the **target/release** folder 

## Examples

You can see information of an IP Address by providing the IP with it's prefix

```bash
ripcalc 192.168.1.10/24 
```

The expected output will show details such as network address and subnet mask.

### Subnetting 

You can also subnet an IP by providing the network address with it's prefix. 

> [!IMPORTANT]
> Subnetting will fail if the IP Address is not a network address.

The **p** option allows to select what should be the new prefix of the subnets.

```bash
ripcalc 192.168.0.0/20 -p 24 
```

The expected output will show all the subnets created. You can also limit the amount of subnet printed out by using the **l** option.

```bash
ripcalc 192.168.0.0/20 -p 24 -l 4 
```

The expected output will show only the first four subnets created.
