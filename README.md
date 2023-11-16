# What is this fork?
## The goal:
CD-MOJ is going to be a complete rewrite of original CD-MOJ, that is supposed to retain utilization compatibility with the original project as much as possible. That means the way contests are created should be virtually the same, and the external APIs should work the same, say for the MOJinho bot that is able to retrieve information from the running instance.

## What can be improved with this fork:
There are a few points that I believe can receive improvements, so I'll list a few:
- CD-MOJ often takes way too long to judge a problem, even when the judging list is empty.
- The running instance often can't keep up with high demand.
- The user experience lacks some polishment, which would need changes on the backend to be properly enhanced.

## How will that be improved:
The backend server will be rewritten in Async Rust, with the use of the Actix-Web framework. This allows the backend server to handle a high volume of requests, with very little overhead. The Actix-RT runtime allows for spawning multiple independent threads that can sleep until they can be processed. This can allow for some control granularity by the sysadmins: a certain number of threads can be reserved for the compiler, while other threads can be reserved to running tests and comparing diffs for judging solutions in parallel, allowing for maximum performance and utilization of resources.
For my plans on enhancing the user experience, I plan on rewriting the API architecture for how solutions are submitted. The idea is to use gRPC, which allows the backend server to better show judgement status, in a more interactive way, possibly showing run time for each test and other statistics that won't bias the correction process.

## When will all that be implemented?
Soon:tm:. I have been studying ways to improve this project for a while, but currently I don't have enough time to commit to this project. Hopefully in December, but probably in January I'll be able to start this project.  

# The original description of the root project:
#A Contest Driven Meta Online Judge

CD-MOJ is a meta online judge, where the main objective is to submit users
codes to another online judge, in order to take advantage of the current
infrastructure available.

Our main goal is to create a simple submission system where teachers can set
contests, or exercise lists, to their students and follow how students are
evolving in an Algorithm class.

Also, we use a simple system to detect software plagiarism.

Besides the idea of creating a simple system for professors, we aim to
create an API to deploy a decentralized infrastructure to judge problems
online.

The idea is to create trustworthy peers to judge problems, in order to avoid
outages of some judge sites. Besides, anyone could set it own instance of
CD-MOJ and could benefit of this model and also to contribute with a judging
machine.

We look forward some students to create and set this distributed model of
judging problems.

##Code

CD-MOJ is entirely written in Bash script, server and html side. It is easy
to maintain and easy to create plugins.

If you would like to contribute to our platform, please feel free to send
bug reports and pull requests.
