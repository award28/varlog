# Learnings

During the development of Varlog, there were numerous successes, failures, and
learnings. In this section we'll review some of these cases and what can be taken
away from them.

## Service Discovery

One of the challenges faced was service discovery for the mesh network
architecture. Initially, attemps were made to use the same approach as the
`arp -a`[^arp] command to discover services on the network. However, piping to the
shell was deemed unreliable in various conditions[^piping]. After researching the
inner workings of arp further, an attempt was made to ping each IP in the host
machines network range and wait for a response, but it proved to be extremely slow
for network ranges with a 16 bit mask. However, `arp` is fast, so this could not
be how it was implemented.

After researching how the address resolution protocol works, a version was finally
implemented which followed the proper protocols approach, using the MAC broadcast
address to request hosts on the same network. The team researched arp further to
understand MAC. While this did work, it only worked for hosts on the same network,
and was ultimately abandoned as an approach. Despite this, much was learned about
service discovery and a potential future improvement which could be added for a
gossip-style protocol. This approach would boot machines with knowledge of all
existing machines and send out their hostname to those existing machines. These
machines would then store the received hostname. The core of the protocol involves
periodic inter-process interactions for health checks of parallel machines, and
this approach may be worth exploring in the future.

## Tail-Based Reverse Log Iteration

Another challenge faced was achieving single digit millisecond lookup on files
larger than 1GB. This problem could have been approached with a top-down scan,
keeping the last n found lines in memory. The problem with this approach is that
it would be slow to reach the end of the file, and Varlog is almost entirely
concerned with only the end of a given file.

The implemented approach is based on the unix `tail`[^tail] command. This algorithm seeks
to the end of the file, and reads backwards in chunks of 10 KB. By reading buffered
bytes backwards, the algorithm discovers new lines and adds them to a queue. This
queue is then used to pop results on demand until the file is exhausted or the number
of lines requested have been read. The one caveat to this approach is that requests
which skip more than half of the file will be slower than an implementation which
reads from the top down.

## Cargo Workspaces

Cargo workspaces were not suitable for this type of project as it resulted in
unecessary dependencies being shared and built for each workspace. This resulted in
extremely slow docker builds which could not properly cache project dependencies,
since they were all dependent on the same `Cargo.lock`. In the future Varlog can be
reworked to separate each of the workspace projects into it's own repo, removing
this issue. Alternatively, the workspace approach could be abandoned in favor of
of workspace acting as it's own root crate.

## Summary

Varlog's development was not without its challenges. There were a number of obstacles
encountered with service discovery, achieving single digit millisecond lookup on large
files, and cargo workspaces. These learnings will inform future work and help improve
Varlog.

With this section concluded, that wraps up the Varlog system overview! If you're
looking to setup Varlog locally, please continue on to the next section. For the API
overview, you can start out with the [Endpoints](v1_endpoints.md) overview.

---

[^arp] [arp.](https://manpages.debian.org/stretch/net-tools/arp.8.en.html)

[^piping] There's no way to be sure that a given machine would have the `arp` command
available. Also, installing the command on the host machine could not be considered
as that is outside of Varlog's control.

[^tail] [The referenced tail implementation.](https://git.savannah.gnu.org/cgit/coreutils.git/tree/src/tail.c#n525)
