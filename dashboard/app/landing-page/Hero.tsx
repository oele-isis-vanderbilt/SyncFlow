import Link from 'next/link';

const Hero = () => {
  return (
    <section className="mx-auto flex max-w-8xl flex-col overflow-hidden px-4 py-6 sm:py-8 lg:pt-16 lg:pb-24 dark:bg-hero-gradient">
      <div className="container mx-auto flex flex-col items-center gap-10">
        <h1 className="max-w-4xl text-center font-extrabold text-4xl text-gray-900 leading-none lg:text-5xl xl:text-6xl dark:text-white">
          <span className="xl:inline">
            Making Real-time Multimodal Applications Possible
          </span>
        </h1>
        <p className="max-w-3xl text-center text-gray-500 text-lg leading-normal lg:text-xl dark:text-gray-400">
          Harmonize your data streams and build real-time multimodal
          applications with ease.
        </p>
        <Link href="#about">
          <button className="items-center gap-1.5 rounded-lg bg-blue-700 p-2.5 font-medium text-lg text-white">
            Get Started -&gt;
          </button>
        </Link>
      </div>
    </section>
  );
};

export default Hero;
