# Subnet

A program to calculate IPv4 subnets based on a given address and the number of devices you want to connect to that network just for fun.

## Usage

```terminal
subnet <ADDR> <MACHINES>
```

## Arguments

```terminal
<ADDR>      IPv4 address
<MACHINES>  Number of machines to implement subnet
```

## Example

### Input

```terminal
subnet 192.168.20.0 120
```

### Output

```terminal
Subnet: 192.168.20.0 | Broadcast: 192.168.20.127 | Mask: /25 | Useful range: [192.168.20.1 - 192.168.20.126]
```
