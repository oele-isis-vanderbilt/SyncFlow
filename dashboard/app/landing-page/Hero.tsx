import Link from 'next/link';

const Hero = () => {
  return (
    <section className="max-w-8xl dark:bg-hero-gradient mx-auto flex flex-col overflow-hidden px-4 py-6 sm:py-8 lg:pb-24 lg:pt-16">
      <div className="container mx-auto flex flex-col items-center gap-10">
        <h1 className="max-w-4xl text-center text-4xl font-extrabold leading-none text-gray-900 lg:text-5xl xl:text-6xl dark:text-white">
          <span className="xl:inline">
            Making Real-time Multimodal Applications Possible
          </span>
        </h1>
        <p className="max-w-3xl text-center text-lg leading-normal text-gray-500 lg:text-xl dark:text-gray-400">
          Harmonize your data streams and build real-time multimodal
          applications with ease.
        </p>
        <Link href="#about">
          <button className="items-center gap-1.5 rounded-lg bg-blue-700 p-2.5 text-lg font-medium text-white">
            Get Started -&gt;
          </button>
        </Link>
      </div>
    </section>
  );
};

export default Hero;
