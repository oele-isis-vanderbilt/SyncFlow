import { Card } from 'flowbite-react';
import Image from 'next/image';

const Features = () => {
  return (
    <section className="mx-auto flex max-w-8xl flex-col overflow-hidden px-4 py-6 sm:py-8 lg:pt-16 lg:pb-24 dark:bg-gray-800">
      <div className="container mx-auto grid grid-cols-1 justify-between gap-10 sm:grid-cols-2 lg:grid-cols-3">
        <Card
          className="max-w-sm"
          renderImage={() => (
            <Image
              width={500}
              height={500}
              src="/landing-page/streaming.png"
              alt="Multimedia Streaming"
            />
          )}
        >
          <h5 className="font-bold text-2xl text-gray-900 tracking-tight dark:text-white">
            Multimedia Live Streaming
          </h5>
          <p className="font-normal text-gray-700 dark:text-gray-400">
            Support multimedia streaming between consumer and IoT devices.
          </p>
          <button className="items-center gap-1.5 rounded-lg bg-blue-700 p-2.5 font-medium text-lg text-white">
            Read More -&gt;
          </button>
        </Card>

        <Card
          className="max-w-sm"
          renderImage={() => (
            <Image
              width={500}
              height={500}
              src="/landing-page/iot.png"
              alt="Iot Integration"
            />
          )}
        >
          <h5 className="font-bold text-2xl text-gray-900 tracking-tight dark:text-white">
            IoT Device Integration
          </h5>
          <p className="font-normal text-gray-700 dark:text-gray-400">
            Incorporate advance IoT sensors to empower and enrich user
            interactions.
          </p>
          <button className="items-center gap-1.5 rounded-lg bg-blue-700 p-2.5 font-medium text-lg text-white">
            Read More -&gt;
          </button>
        </Card>

        <Card
          className="max-w-sm"
          renderImage={() => (
            <Image
              width={500}
              height={500}
              src="/landing-page/ai-deployment.png"
              alt="AI Deployment"
            />
          )}
        >
          <h5 className="font-bold text-2xl text-gray-900 tracking-tight dark:text-white">
            Full-Stack AI Deployment
          </h5>
          <p className="font-normal text-gray-700 dark:text-gray-400">
            Use AI models in the edge, client-side, and in the cloud.
          </p>
          <button className="items-center gap-1.5 rounded-lg bg-blue-700 p-2.5 font-medium text-lg text-white">
            Read More -&gt;
          </button>
        </Card>
      </div>
    </section>
  );
};

export default Features;
