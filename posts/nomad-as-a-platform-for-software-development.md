# Nomad as a platform for software development

At Entropy1729, we believe that product development teams should have **ownership** over the infrastructure that runs their services and **autonomy** to operate it:

* engineers should be able to deploy their changes to production every time they merge a pull-request –if not even before, during development;
* the ceremony for adding a new environment or changing the setup should be minimized;
* infrastructure decisions should be driven by the target architectural characteristics for the software rather than by operational restrictions.

At the same time, DevOps work can get arbitrarily complex and overwhelming and, if care is not taken, software engineers will easily end up spending more time in operations than in product development, duplicating efforts and failing to come up with consistent solutions across the organization. In order to strike the right balance, we decided to invest early in forming a [Platform Team](https://blog.pragmaticengineer.com/platform-teams/) to provide the building blocks to enable the rest of the teams while minimizing operational burden.

And key to realizing this vision is the choice to use [Nomad](https://www.nomadproject.io/), a scheduler and container orchestrator by HashiCorp, sort of a simpler alternative to Kubernetes. Nomad decouples Infra setup and provision from service resource allocation, deploy and execution: product development teams write Dockerfiles for their services and work together with the Platform Team to define a [Nomad job specification](https://www.nomadproject.io/docs/job-specification) (amount of instances, required resources, deploy strategy, etc).

The Platform Team can then use this as a framework for providing standardized solutions for common requirements like environments, configuration, deploys, secrets management, logging, metrics and monitoring. A nice side-effect is that by using Nomad we also get some degree of abstraction over the underlying infrastructure, so we can mix and switch between cloud and bare-metal providers with reduced impact on the application software.

As this project continues to grow, we will continue to share our insights and tutorials in this blog.
