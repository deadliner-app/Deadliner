import React from "react";
import Section from "../components/Section";

const Motivation = () => {
  return (
    <Section id="motivation" className="items-start justify-start flex-col">
      <h1 className="font-semibold text-8xl text-white mb-14">Motivation</h1>
      <p className="text-grey text-xl pb-7">
        So, I was researching on “what makes someone productive?” for the 731th
        time! And surprisingly I found it in a video from{" "}
        <a
          className="text-white font-medium hover:underline"
          href="https://www.youtube.com/c/JonasTyroller"
        >
          Jonas Tyroller
        </a>{" "}
        - Indie game dev.
      </p>
      <p className="text-grey text-xl pb-7">
        I like to take inspiration from game devs cause in my opinion
        they&apos;re the most motivated/inspiring developers 😊.
      </p>
      <p className="text-grey text-xl pb-7">
        This video was about asking successful game devs on how they think about
        productivity and the results were <strong>fascinating</strong>, they all
        agreed on <strong>Deadlines</strong> as the most effective solution for
        procrastination.
      </p>
      <p className="text-grey text-xl pb-7">
        I’ve always avoided deadlines in my side projects as they’re usually for
        learning purposes cause I thought setting deadlines will make me rush
        through the project and skip learning important details.
      </p>
      <p className="text-grey text-xl pb-7">
        And so I said to myself: “Hey, let’s test this technique on my current
        side project(lightning-fast markdown parser), that can also improve my
        deadlines’ estimations a bit.”
      </p>
      <p className="text-grey text-xl pb-7">
        Then I set a 14-days deadline, I immediately started to feel the
        difference on the first 2-4 days, after that I started to forget about
        how much time is left, so I wake up and say: “Hey, let’s watch a YT
        video and get to work after that, not a big deal right?”, plot twist:
        Yes, it was a big deal. The way you start your day has a huge impact on
        how the rest of it will go.
      </p>
      <p className="text-grey text-xl pb-7">
        So I searched for desktop apps for setting deadlines and keeping track
        of it and what I found was that they’re all share the same issues:
      </p>
      <ul className="text-grey list-disc list-inside text-xl pb-7">
        <li>Limited customizations</li>
        <li>I’ve to start it whenever I boot up my OS</li>
        <li>Lots of super annoying notifications for the remaining time</li>
        <li>Uses kind of a lot of resources in the background</li>
      </ul>
      <p className="text-grey text-xl pb-7">
        So, that was a sign for a good Rust 🦀 project that aims for solving
        these issues 💪.
      </p>
    </Section>
  );
};

export default Motivation;
