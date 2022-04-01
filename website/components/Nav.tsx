import Image from "next/image";
import React from "react";
import { Link } from "./Link";

export const Nav = () => {
  return (
    <header className="w-full flex justify-center z-50 top-0 transition duration-200">
      <nav className="flex justify-between items-center w-full py-4 lg:py-6 max-w-screen-2xl px-6 lg:px-12">
        <div>
          <Image src="/logo.svg" width={210} height={50} alt="Logo" />
        </div>
        <ul className="grow list-none flex justify-end">
          <li className="mb-8 ml-10 lg:mb-0 ">
            <Link href="/#features" text="Features" />
          </li>
          <li className="mb-8 ml-10 lg:mb-0 ">
            <Link href="/#gallery" text="Gallery" />
          </li>
          <li className="mb-8 ml-10 lg:mb-0 ">
            <Link href="/#motivation" text="Motivation" />
          </li>
          <li className="mb-8 ml-10 lg:mb-0 ">
            <Link href="/#downloads" text="Downloads" />
          </li>
        </ul>
        <a href="https://github.com/YassinEldeeb/Deadliner">
          <div className="ml-10 w-8 h-8">
            <Image src="/images/github.png" alt="" width={32} height={32} />
          </div>
        </a>
      </nav>
    </header>
  );
};
