# Design Choices: Tradeoffs Made

In this section, we'll discuss the design choices made for Varlog and the
tradeoffs associated with them. We'll begin by examining the core language
used to build Varlog, which is Rust. We'll explore the advantages and
disadvantages of using Rust as the core language for a Varlog. Then, we'll
move on to compare the benefits of using a REST API Sidecar an alternative
like a sidecar agent. Finally, we'll delve into the tradeoffs between on-demand
log aggregation as compared to  a storage layer for low-impact retrieval.
By analyzing these design choices, we can gain a deeper understanding of
Varlog's architecture and the reasoning behind its implementation.

## Rust as the Core Language

Rust was chosen as the core language for Varlog due to its speed, memory safety,
and thread safety, making it an excellent choice for a log aggregation tool.
Additionally, Rust's control over memory usage leads to code that runs quickly
and efficiently, which is critical for a tool like Varlog. Cross-platform support
was another deciding factor, as Rust's ability to compile to various platforms and
architectures allowed us to create a tool that can be run on any operating system.

While Rust offers many benefits, it does have a steep learning curve compared to
other programming languages. Developers unfamiliar with Rust may face challenges
when getting started with contriubting to Varlog. Additionally, compared to other
languages like Java or Python, the Rust community is relatively small. This may
result in fewer libraries or resources available to developers, which could impact
Varlog's development timeline and future maintenance efforts.

| Advantages | Disadvantages |
|-|-|
| Performance | Smaller developer community |
| Memory safety and thread safety |  Steep learning curve |
| Cross-platform support | Fewer libraries or resources available to developers |


## Sidecare REST API

The Sidecar REST API was chosen for Varlog for a number of reasons. REST API's are
flexible, familiar, and easy to implement, which can save developers time and effort
integrating with the Varlog API. Additionally, the ability to query logs on an
as-needed basis using the existing log files results in no storage requirements!

However, there are some disadvantages to this approach. Latency may be an issue due
to the requirement for round trips to the server for log requests which are too large.
This would require more bandwidth and processing power required compared to something
like a sidecar agent, which would send logs to an external data warehouse as they're
generated. However, since no external storage as a requirement was a goal of Varlog,
this approach could not be pursued further. One benefit of the agent approach would be
that the processing requirements for the agent would scale with the rate at which logs
are written. With the REST API sidecar, the servers scale at the rate with which logs
are requested. While this can be mitigated with rate limiting, it is still a risk.

| Advantages | Disadvantages |
|-|-|
| Ease of Integration | Scales external to normal server workload |
| Flexibility | Latency |
| Familiarity | Round trips for large requests |
| No Additional Storage Costs |  |

## On-Demand Log Retrieval

Varlog's on-demand log aggregation was chosen for its zero-cost storage requirements
and quick retrieval of only relevant logs. Aggregating logs on-demand requires fewer
resources than storing all logs. This makes getting started with Varlog simple, since
users can add Varlog to their existing infrastructure and immediately start benefiting
from log management. As compared to something like a storage layer, Varlog has access
to all of the existing logs for any given system, since it's not only sending new log
events.

However, on-demand log aggregation may comes with real-time limitations, and it
requires additional overhead to retrieve the logs. As covered in the sidecar comparison,
this type of log retrieval results in the sidecar load not scaling at the same rate
as the main servers workload. This issue would not be a problem with a storage layer,
since the logs could be streamed to the storage layer as they're written.

| Advantages | Disadvantages |
|-|-|
| Zero-cost storage requirements | Real-time limitations |
| Quick retrieval of relevant logs | Additional overhead |
| Access to all existing logs | Scales external to normal server workload |

Overall, the design choices made for Varlog were carefully considered to balance the
advantages and disadvantages of each approach. In the next section of the
documentation, we'll explore some of the learnings discovered while building Varlog.
