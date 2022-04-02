import Link from "next/link";
import React from "react";

const Footer = () => {
  return (
    <p className="text-white text-md lg:text-2xl text-center py-10 opacity-80">
      Made with ❤️ by{" "}
      <span className="font-medium hover:underline">
        <Link href="https://github.com/YassinEldeeb">Yassin Eldeeb</Link>
      </span>
    </p>
  );
};

export default Footer;
