import type { NextPage } from "next";
import { Nav } from "../components/Nav";
import { Separator } from "../components/Separator";
import Features from "./Features";
import Gallery from "./Gallery";
import Hero from "./Hero";
import Motivation from "./Motivation";
import Downloads from "./Downloads";
import Footer from "../components/Footer";

const Home: NextPage = () => {
  return (
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
  );
};

export default Home;
