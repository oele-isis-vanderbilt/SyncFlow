export interface DocsSidebarSection {
  title: string;
  items: DocsSidebarItem[];
}

export interface DocsSidebarItem {
  title: string;
  href: string;
  isNew?: boolean;
  isExternal?: boolean;
}

export const DOCS_SIDEBAR: DocsSidebarSection[] = [
  {
    title: 'introduction',
    items: [
      {
        title: 'Introduction',
        href: '/docs/introduction/introduction',
      },
      {
        title: 'Livekit',
        href: '/docs/introduction/livekit',
      },
      {
        title: 'Architecture',
        href: '/docs/introduction/architecture',
      },
      {
        title: 'Registration and Account Creation',
        href: '/docs/introduction/registration',
      },
    ],
  },
  {
    title: 'Release Information',
    items: [
      {
        title: 'Release Notes',
        href: '/docs/release/release-information',
      },
    ],
  },
  {
    title: 'Tutorials',
    items: [
      {
        title: 'Projects and Resources',
        href: '/docs/tutorials/projects',
      },
      {
        title: 'Integration Guide',
        href: '/docs/tutorials/integration-guide',
      },
      {
        title: 'Non Login Client Interaction',
        href: '/docs/tutorials/non-login-client-interaction',
      },
    ],
  },
  {
    title: 'The SyncFlow EcoSystem',
    items: [
      {
        title: 'SyncFlow IOT App',
        href: '/docs/syncflow-ecosystem/syncflow-iot-app',
      },
      {
        title: 'SyncFlow Text Egress Actor',
        href: '/docs/syncflow-ecosystem/syncflow-text-egress-actor',
      },
      {
        title: 'SyncFlow Node Client',
        href: '/docs/syncflow-ecosystem/syncflow-node-client',
      },
      {
        title: 'SyncFlow Python Client',
        href: '/docs/syncflow-ecosystem/syncflow-python-client',
      },
    ],
  },
  {
    title: 'How to use SyncFlow',
    items: [
      {
        title: 'Researchers',
        href: '/docs/how-to/researchers',
      },
      {
        title: 'Developers',
        href: '/docs/how-to/developers',
      },
      {
        title: 'Client Development',
        href: '/docs/how-to/client-development',
      },
    ],
  },
  {
    title: 'Deploying SyncFlow',
    items: [
      {
        title: 'Dockerized Deployment',
        href: '/docs/deployment/dockerized',
      },
      {
        title: 'Local Deployment',
        href: '/docs/deployment/local',
      },
    ],
  },
  {
    title: 'Contributing Guidelines',
    items: [
      {
        title: 'Contributing',
        href: '/docs/contributing/contributing-guidelines',
      },
    ],
  },
  {
    title: 'Funding and Acknowledgements',
    items: [
      {
        title: 'Funding Information',
        href: '/docs/funding/funding-info',
      },
    ],
  },
];
