#include <stdio.h>
int main()
{
    int a, b, c ,d ,e ,f ,g, h;
    a = 1;
    b = c = d = e =f =g = h = 0;

/*set b 81      */     //b = 81;
/*set c b       */     //c = b;
/*jnz a 2       */     //if (a) goto A;
/*jnz 1 5       */     //goto B;
/*mul b 100     */  //A: b *= 100;
/*sub b -100000 */     //b -= -100_000;
/*set c b       */     //c = b;
/*sub c -17000  */     //c -= -17000;
/*set f 1       */  B: f = 1;
/*set d 2       */     d = 2;
/*set e 2       */  // E: e = 2;
/*set g d       */  //D: g = d;
/*mul g e       */  //   g *= e;
/*sub g b       */  //   g -= b;
/*jnz g 2       */     //if (g) goto C;
/*set f 0       */     //f = 0;
/*sub e -1      *///C: e -= -1;
/*set g e       *///     g = e;
/*sub g b       *///     g -= b;
/*jnz g -8      */ //    if (g) goto D;
/*sub d -1      */   //  d -= -1;
/*set g d       */   //  g = d;
/*sub g b       */   //  g -= b;
/*jnz g -13     */   //  if (g) goto E;
/*jnz f 2       */     //if (f) goto F;
/*sub h -1      */     //h -= -1;
/*set g b       */  //F: g = b;
/*sub g c       */     // g -= c;
/*jnz g 2       */     //if (g) goto G;
/*jnz 1 3       */     //goto DONE;
/*sub b -17     *//*G */ //b -= -17;
/*jnz 1 -23     */     // goto B;
DONE: printf("%d\n", h);
      return 0;
}
