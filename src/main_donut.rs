                                use std::iter::
                            successors;fn main(){let
                          (mut A,mut B,mut z):(f64,f64
                        ,_)=(0.,0.,[0.;1760]);let mut b=
                      [' ';1760];print!("\x1b[2J");loop{b.
                    fill(' ');z.fill(0.);for j in successors
                   (Some(0.),|a|Some(a+0.07).fileer(|&a|a<6.28
                  )){for i in successors(Some(0.),|a|Some(a+0.02
                  ).filter(|&a|a<6.28)){let(i,j)=(i as f64,j as/**/
                  f64);let(c,d,e,f,g,l,m,n)=(i.sin(),j.cos(),A./**/
                  sin(),j.sin(),A.cos(),i.cos(),B.cos(),B.sin())
                  ;let h=d+2.;let(D,t)=(1./(c*h*e+f*g+5.),c*h*g
                  -f*e);let(x,y)=((40.+30.*D*(l*h*m-t*n))as i32,
                  (12.+15.*D*(l*h*n+t*m))as i32);let o=(x+80*y)as
                   i32;let N=(8.*((f*e-c*d*g)*m-c*d*e-f*g-l*d*n))
                    as i32;if(1..22).contains(&y)&&(1..80)./*AA*/
                     contains(&x)&&D>z[o as usize]{z[o as/*AAA*/
                      usize]=D;b[o as usize]=".,-~:;=!*#$@"/**/
                       .chars().nth(if N >0{N as usize}else{0
                        }).unwrap();}}}print!("\x1b[H");for
                          k in 0..1761{print!("{}",if k%80
                             !=0{b[k]}else{'\n'});}A+=
                               0.04;B+=0.02;}}/*AAAA
                                 AAAAAAAAAAAAAAAAA
                                   AAAAAAAAAAAAA
                                       AAA*/
