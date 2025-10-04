# Supplemental RV Cameras
The motivation for this project began with a blowout. We had just bought a new Jeep Grand Cherokee, and were were
towing it for the first time. Unfortunately, this version of the Jeep has a steering issue when neutral towing
within 5 minutes of any interaction that keeps it "awake." Until it goes to "sleep," the steering wheel tends not to
turn. As a result, we ended up driving for quite a few miles after breaking the tire bead on a curb, shredding the
tire and causing some expensive damage to our brand new Jeep.

The stock camera at the top of the RV can't see the tires on the Jeep. In addition, the sight distance of that camera
is limited, and it can be difficult to judge vehicles in adjacent lanes or placement of the towed vehicle in confined
turns. So before we towed the Jeep from the West Coast all the way across the US, we decided to build a pair of
supplemental cameras. For my first build, I decided to buy a pair of POE cameras, and use a mini computer to
stream the images to a monitor on the RV dashboard. Given the success of the POE cameras, I'm planning to explore 
building a second pair of cameras using Raspberry Pi and something less expensive for the display.

But first, here is how I build the initial version.

- 2 x [Revodata HD 5MP Mini PEI IP Camera](https://www.amazon.com/dp/B0B1LQ5THS)
- 1 x [Linovision 5 Port Gigabit PoE Switch](https://www.amazon.com/dp/B09HGWLZSD)
- 1 x [12v to 19v 5A 95W DC Boost Coverter](https://www.amazon.com/dp/B0B7481QCY)
- 1 x [14" 4K UHD IPS 3840x1100 Stretched Bar Touch Screen Monitor](https://www.amazon.com/dp/B0BXL1NXVS)
- 1 x [Cat6e Ethernet Internet Cable - 100 FT](https://www.amazon.com/dp/B0BG3BZ9VT)
- x x [Mini PC with Intel 12 Gen Core i3-1220P](https://www.amazon.com/Beelink-SEI12-Generation-Processors-i5-1235U)

The mini PC shown above is not the one I used on our RV trip. If I didn't already have a mini PC lying around, this is
one I would have considered purchasing. Hardware support for H.265/HVEC and AVI is ideal for decoding the video streams
from the camera. Since I was working with an older PC, I was limited to using H.264, which is less efficient poorer
quality. Ultimately, the video quality was acceptable, though the power demands of the system were a bit too much when
the alternator was having difficulty keeping up with charging nearly-dead RV batteries as well.
