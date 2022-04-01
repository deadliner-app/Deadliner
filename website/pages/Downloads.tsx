import Image from "next/image";
import { useRouter } from "next/router";
import React, { useEffect, useState } from "react";
import Section from "../components/Section";

const Downloads = () => {
  let [link, setLink] = useState(null);
  let router = useRouter();

  useEffect(() => {
    fetch("https://api.github.com/repos/YassinEldeeb/deadliner/releases/latest")
      .then((data) => data.json())
      .then((data) => {
        setLink(data.html_url);
      });
  }, []);

  return (
    <Section id="downloads" className="justify-start flex-col">
      <h1 className="font-semibold text-8xl text-white mb-14">Downloads</h1>

      {link ? (
        <a href={link}>
          <div className="flex cursor-pointer decoration-white" title={link}>
            <Image src="/images/link.png" width={41} height={41} alt="" />
            <h2 className="ml-4 text-white font-medium text-3xl hover:underline">
              Follow the latest github release for your Operating System.
            </h2>
          </div>
        </a>
      ) : (
        <p className="text-white text-2xl">Fetching latest github release...</p>
      )}
    </Section>
  );
};

export default Downloads;
