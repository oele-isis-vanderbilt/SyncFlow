# SyncFlow

<p align="center">
  <img src="./dashboard/public/syncflow.png" alt="SyncFlow" />
</p>

A server manager for [LiveKit](https://livekit.io/), built for Multimodal Learning Analytics (MMLA) applications. SyncFlow provides scalable, robust cloud infrastructure for automated MMLA deployments, making it easier to integrate real-time multimodal data collection into AI-powered computer-based learning environments.

> **Status:** Production Ready (Beta). Core functionality is stable and usable, but expect breaking changes as the API and architecture continue to evolve.

This project has two main parts:

- **Dashboard** — a Next.js application used to manage the server.
- **Server** — an Actix Web application used to control and manage the LiveKit services.

## Hosted Option

Don't want to self-host? Sign up at [syncflow.live](https://syncflow.live) and start using SyncFlow by bringing your own streaming (LiveKit) and storage (S3) resources.

## Installation

SyncFlow is fully containerized. The only prerequisite is [Docker](https://docs.docker.com/get-docker/) (with Docker Compose). For deployment of SyncFlow, we have a dedicated respository providing instructions for deploying SyncFlow on any virtual machine or cloud provider: [syncflow-deploy](https://github.com/oele-isis-vanderbilt/syncflow-local-deployment)

## License

This project is licensed under the [Apache License 2.0](./LICENSE).

## Citation

If you use SyncFlow in your research, please cite:

```bibtex
@inproceedings{timalsina2025syncflow,
  title     = {SyncFlow: A Scalable Platform for Multimodal Learning Analytics},
  author    = {Timalsina, Umesh and Davalos Anaya, Eduardo and Sanda, Nihar and
               Zhang, Yike and Horn Fonteles, Joyce and T S, Ashwin and Biswas, Gautam},
  booktitle = {Proceedings of the US Research Software Engineer Conference (US-RSE'25)},
  year      = {2025},
  address   = {Philadelphia, PA},
  doi       = {10.5281/zenodo.17254182},
  url       = {https://doi.org/10.5281/zenodo.17254182}
}
```

## Funding Information

This work is supported by the National Science Foundation under Grant No. DRL-2112635.
