import type { NextPage } from "next";
import { Nav } from "../components/Nav";
import { Separator } from "../components/Separator";
import Features from "./Features";
import Gallery from "./Gallery";
import Hero from "./Hero";
import Motivation from "./Motivation";
import Downloads from "./Downloads";
import Footer from "../components/Footer";
import Head from "next/head";

const Home: NextPage = () => {
  return (
    <>
      <Head>
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:title" content="Deadliner" />
        <meta
          name="twitter:description"
          content="Deadliner helps you keep track of the time left for your deadline by dynamically updating the wallpaper of your desktop with the time left."
        />
        <meta name="twitter:image" content="/images/web-screenshot.png" />

        <title>Deadliner</title>
      </Head>

      <div className="bg-fixed bg-gradient-to-tr from-radial-tr to-radial-bl">
        <div className="flex flex-col h-screen">
          <Nav />
          <Hero />
        </div>
        <Separator />
        <Features />
        <Separator />
        <Gallery />
        <Separator />
        <Motivation />
        <Separator />
        <Downloads />
        <Separator />
        <Footer />
      </div>
    </>
  );
};

export default Home;
