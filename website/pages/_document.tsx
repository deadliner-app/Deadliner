import { Html, Head, Main, NextScript } from "next/document";

export default function Document() {
  return (
    <Html>
      <Head>
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" />
        <link
          href="https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600&display=swap"
          rel="stylesheet"
        />
        <link rel="icon" type="image/x-icon" href="/icon.ico"></link>
      </Head>
      <body className="font-Poppins">
        <Main />
        <NextScript />
      </body>
    </Html>
  );
}
