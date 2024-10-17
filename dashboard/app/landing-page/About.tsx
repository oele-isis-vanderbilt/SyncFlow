import React from 'react';

const About = () => {
  return (
    <section
      id="about"
      className="mx-auto flex max-w-8xl flex-col overflow-hidden px-4 py-6 sm:py-8 lg:pt-16 lg:pb-24 dark:bg-gray-800"
    >
      <div className="container mx-auto flex flex-col items-center gap-10">
        <h1 className="max-w-4xl text-center font-extrabold text-3xl text-gray-900 leading-none lg:text-3xl xl:text-4xl dark:text-white">
          <span className="xl:inline">
            Orchestrator Platform for Mutltimodal Data Collection/Analytics
          </span>
        </h1>
        <p className="max-w-3xl text-center text-gray-500 text-lg leading-normal lg:text-xl dark:text-gray-400">
          Our Platform is designed to provide you all the batteries you need to
          deliver/monitor AI driven multimodal applications in realtime
        </p>
      </div>
    </section>
  );
};

export default About;
