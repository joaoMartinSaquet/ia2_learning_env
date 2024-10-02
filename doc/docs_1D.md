
# Games states transition machine
```mermaid
stateDiagram-v2
    [*] --> Started : s
    Started --> Running : s
    Running --> Paused : s
    Paused --> Running : s
    Paused --> End : e 
    Running --> End : end_timer
    End --> Started : r
```
