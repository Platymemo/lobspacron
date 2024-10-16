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

2. Make sure your Hazelcast Cluster is configured to allow REST API calls
```xml
<hazelcast>
    ...
    <network>
        <rest-api enabled="true">
            <endpoint-group name="DATA" enabled="true"/>
        </rest-api>
    </network>
    ...
</hazelcast>
```

3. Start a Hazelcast Cluster (1 node is fine)
```bash
bin/hz-start
```

4. Open the project directory in multiple terminals and start a host and client processes
```cmd
cargo run -- host
```
```cmd
cargo run
```