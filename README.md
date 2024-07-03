# Rust Based Distributed KV Store
Thanks for checking out the project I am building here!
## Why?
So when we have so many other KV Stores, why build one yourself.
Well ...
1) Its always great to build things yourselves to learn how things work.
Nowadays everything is built for us, and its hard to learn the basics of binding
to server ports, creating tcp listeners and all that.
2) Distributed Systems is the future, so learning fundamental consensus
algorithms is very necesary
3) Building large scale systems is fun, instead of using time leetcoding, we can
be building systems that may not have a direct usecase now, but some time when
your sitting there in your room thinking about something else to build you find
the tools laying around.

## What?
So what is it that I am building here:

Firstly it is a Key Value Store: Essentially a hashmap that is saved over time. 
That simple! 
It doesnt have tables, relations, graph connections, foriegn keys.
You give it a key, and you get a value.

Why are these needed: Becase sometimes data collection does not need to be
complicated, but instead condensed and simplified for easy reading, modifying,
and accessing.

Now what's so different about this system. Well first of all its made to be
scalable, so it connects to other servers, but also it understands the
importance of replication and consensus.

We need multiple servers because servers can fail, and we want our data and
changes to still persist. Now if we have a bunch of servers always taking
changes and accepting responses, they will now have a consensus problem. How do
we fix that Raft!

That is what I building here, a key value system with a raft consensus model on
top of it!

This code may not be the best ... it goes through revisions all the time and is
something I want others to look at and learn something from: Whether something I
did good or smth I did bad.

## Progress

The KV Store works great, saves information and is a super simple
implementationt that works perfectly for the job

The Servers Can all talk to each other and the Raft Nodes too know how to talk
to each other

TODO: Well the raft nodes seem to all never want to be the leader. What a great
sense of modesty I have enabled into these nodes :), but right now i need
something to take charge. So to the workshop it is

## Contact
Please let me know if you have any suggestions. This is my first large scale
rust project and also the biggest networking projet I have done aswell. Also as
much as that damn Raft Paper talks about how simple it is compared to Paxos ...
SIMPLE is the key word. OMG did it take so many conversations with friends and
online papers and videos to figure out anything. I mean its still not finished
... my consensus stuff is still a work in progress and I am always deleting and
restarting when I learn something new. So please contact me with any
suggestions at:
aniruddh.anand@utexas.edu
