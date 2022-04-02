import React from "react";
import { Navigation, A11y, Scrollbar } from "swiper";
import { SwiperSlide, Swiper } from "swiper/react";
import Section from "../components/Section";
import Image from "next/image";

import "swiper/css";
import "swiper/css/navigation";
import "swiper/css/scrollbar";

let galleryImages: string[] = [
  "screenshot1.png",
  "screenshot2.png",
  "screenshot3.png",
  "screenshot4.png",
  "screenshot5.png",
  "screenshot6.png",
];

const Gallery = () => {
  return (
    <Section id="gallery">
      <Swiper
        spaceBetween={10}
        slidesPerView={1}
        modules={[Navigation, A11y, Scrollbar]}
        navigation
        scrollbar={{ draggable: true }}
      >
        {galleryImages.map((e) => (
          <SwiperSlide key={`gallery-image-${e}`}>
            <div className="pointer-events-none select-none">
              <Image
                objectFit="contain"
                src={`/images/${e}`}
                width={1920}
                height={1080}
                alt="desktop screenshot"
              />
            </div>
          </SwiperSlide>
        ))}
      </Swiper>
    </Section>
  );
};

export default Gallery;
