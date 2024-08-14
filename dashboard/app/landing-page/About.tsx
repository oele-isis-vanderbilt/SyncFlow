import React from 'react';

const About = () => {
  return (
    <section
      id="about"
      className="max-w-8xl mx-auto flex flex-col overflow-hidden px-4 py-6 dark:bg-gray-800 sm:py-8 lg:pb-24 lg:pt-16"
    >
      <div className="container mx-auto flex flex-col items-center gap-10">
        <h1 className="max-w-4xl text-center text-3xl font-extrabold leading-none text-gray-900 dark:text-white lg:text-3xl xl:text-4xl">
          <span className="xl:inline">
            Orchestrator Platform for Mutltimodal Data Collection/Analytics
          </span>
        </h1>
        <p className="max-w-3xl text-center text-lg leading-normal text-gray-500 dark:text-gray-400 lg:text-xl">
          Our Platform is designed to provide you all the batteries you need to
          deliver/monitor AI driven multimodal applications in realtime
        </p>
      </div>
    </section>
  );
};

export default About;
