import Image from "next/image";
import React, { useEffect, useState } from "react";
import { Link } from "./Link";

export const Nav = () => {
  const [openDrawer, setOpenDrawer] = useState(false);

  const OpenDrawerHandler = async () => {
    setOpenDrawer(!openDrawer);
  };

  useEffect(() => {
    if (openDrawer) {
      document.body.style.overflowY = "hidden";
    } else {
      document.body.style.overflowY = "auto";
    }
  }, [openDrawer]);

  const DesktopNavLinks = () => {
    return (
      <>
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
          <div className="ml-10 w-8 h-8 hover:opacity-70 transition-all duration-100">
            <Image src="/images/github.png" alt="" width={32} height={32} />
          </div>
        </a>
      </>
    );
  };

  const MobileNavLinks = () => {
    return (
      <>
        <ul className="grow list-none flex justify-around flex-col">
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
          <div className="ml-10 mt-3 w-8 h-8 hover:opacity-70 transition-all duration-100">
            <Image src="/images/github.png" alt="" width={32} height={32} />
          </div>
        </a>
      </>
    );
  };

  return (
    <header className="w-full flex justify-center z-50 top-0 transition duration-200">
      <nav className="flex justify-between items-center w-full py-6 max-w-screen-2xl px-8 lg:px-16">
        <div className="cursor-pointer w-44 lg:w-52">
          <Image src="/logo.svg" width={210} height={50} alt="Logo" />
        </div>

        <div
          onClick={OpenDrawerHandler}
          className="flex flex-col cursor-pointer p-3 pr-0 lg:hidden z-50"
        >
          {openDrawer ? (
            <svg
              xmlns="http://www.w3.org/2000/svg"
              className="h-6 w-6 transform scale-150"
              fill="none"
              viewBox="0 0 24 24"
              stroke="#111314"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          ) : (
            <svg
              xmlns="http://www.w3.org/2000/svg"
              className="h-6 w-6 transform scale-150"
              fill="none"
              viewBox="0 0 24 24"
              stroke="#FED843"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M4 8h16M4 16h16"
              />
            </svg>
          )}
        </div>

        <div
          onClick={OpenDrawerHandler}
          className={`py-48 flex flex-col lg:hidden w-screen h-screen fixed left-0 top-0 transition duration-200 bg-[#FABE2C] ${
            openDrawer ? "translate-x-0" : "translate-x-full"
          }`}
        >
          <MobileNavLinks />
        </div>

        <div className="hidden lg:flex">
          <DesktopNavLinks />
        </div>
      </nav>
    </header>
  );
};
