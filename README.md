# Lobspacron

LOwer Bound for and Same PArity as the CROssing Number

Developed by: Edward H

## Introduction

This is an incomplete attempt at calculating the crossing number of the complete bipartite graph K9,9.

## Requirements

Rust v1.81.0
Hazelcast v5.5.0

## How to run

1. Clone this repository
```git
git clone https://github.com/platymemo/lobspacron
```

2. Start a Hazelcast Cluster (1 node is fine)
```bash
bin/hz-start
```

3. Open the project directory in multiple terminals and start a host and client processes
```cmd
cargo run -- host
```
```cmd
cargo run
```