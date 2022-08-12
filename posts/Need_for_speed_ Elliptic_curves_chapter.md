# Need for speed: Elliptic curves chapter

## Introduction

Elliptic curves (EC) have gained widespread acceptance as tools for cryptography. They offer several advantages over other methods, such as RSA, providing equal levels of security with shorter keys (for example, 228-bit keys in EC cryptography are as good as 2300-bit RSA keys). This represents an advantage, since more and more cryptography is done on smart-phones, which are less powerful than computers. These are curves defined by the equation $y^2=x^3+ax+b$ over some [field](https://en.wikipedia.org/wiki/Field_(mathematics)) (for example, the real numbers). Their shape depends on $a$ and $b$, but they look more or less like the following picture:
![An elliptic curve over the real numbers](https://i.imgur.com/rk6M8y0.jpg)

In cryptography, we are not interested in curves defined over the real numbers. We work with them over some finite field $\mathcal{F}_p$ (that is, a set with a finite number of elements, such as $53$, $101$ o $2^{255}-19$), because that gives us a mathematical structure (a [finite group](https://en.wikipedia.org/wiki/Finite_group)) which is very convenient. The curve looks like scattered points with no clear pattern over a finite field:
![](https://i.imgur.com/sQDajke.jpg)

Elliptic curves play a role in key exchange when connecting via SSH to a server or to prove ownership in bitcoin. They also appear when performing digital signatures, generating random numbers (though there have been some problems) and they are useful even to factor numbers ([Lenstra's algorithm](https://en.wikipedia.org/wiki/Lenstra_elliptic-curve_factorization)). For example, in the elliptic curve digital signature algorithm (ECDSA) you have these steps (don't worry if you do not understand all the terms now, we will cover them one by one afterwards):
1. Calculate E=hash(message), where hash is a secure [hash function](https://en.wikipedia.org/wiki/Hash_function).
2. Take Z equal to the $n$ leftmost bits of E, where $n$ is the order of the group (that is, the number of elements making the group).
3. Select a cryptographically secure random number $k$ (never use the same $k$ twice or you'll be revealing your key).
4. Evaluate $(x_1,y_1)=kg$, where $g$ is the generator of the group.
5. Let $r=x_1$.
6. Evaluate $s=k^{-1}(Z+rs_k) \pmod{n}$, where $s_k$ is the secret key.
7. The signature is the pair $(r,s)$.

In this example, we have to evaluate in step 4 an addition on the curve to arrive at point $(x_1,y_1)$, which gives us $r$. In general, $k$ is a large number (having 256 bits, for example), so that operation can be quite expensive. Besides, if the implementation is not done properly, elliptic curve cryptography could be targeted by side-channel attacks, such as timing and cache attacks. Some elliptic curves have properties that allow for a constant-time implementation, which makes them resistant to these strategies.

Elliptic curves also appear in zk-SNARKs (zero-knowledge succint non-interactive arguments of knowledge; we'll go hunting for the SNARK on another post) to provide homomorphic hiding. The word sounds important, but the idea behind is simple. Suppose that there are two variables, $x$ and $y$ and you want (or need) to know $x+y$. The problem is, you don't know them directly, but you have their encrypted form $E(x)$ and $E(y)$. If you have homomorphic hiding, you can compute $E(x+y)=E(x)\times E(y)$, where $\times$ is the operation over the encrypted variables. So, even if you don't know the variables themselves, you can perform mathematical operations on them (and luckily, that's just what you need). This is achieved in practice by means of two elliptic curves (known as a pairing; not all elliptic curves are that sociable or get along quite well with others). To be a good match, we need that the operations can be performed as quickly as possible (among other things). A simple example is the exponential function, $f: \mathbb{R} \rightarrow \mathbb{R}^+/ f(x)=\exp(x)$. If you have $x=2.303$, $\exp(2.303)\approx 10$, $y=3$, $\exp(3)\approx 20.09$, then $\exp(x+y)=\exp(x)\exp(y)=10\times 20.09=200.9$, which is equal to $\exp(5.303)$ and $x+y=5.303$. Of course, in this case it is very easy to go back and know the exact numbers $x$, $y$ and $x+y$; in the case of elliptic curves, this is very hard, owing to the particular group structure. 

To be able to work with elliptic curves, we need to define an operation involving the points on the curve. We can do this using the chord-and-tangent construction: given two points on the curve, we can draw a line connecting them; the line intersects the curve at a third point and we reflect it around the $x$-axis to obtain the sum (remember the picture of the curve defined over real numbers). The formulae are
$s=\frac{y_2-y_1}{x_2-x_1}$
$x_3=s^2-x_1-x_2$
$y_3=s(x_1-x_3)-y_1$

There are some special cases, such as when we want to add a point to itself (we call that "doubling"). To make things work, we need to add a special point $\mathcal{O}$, the point at infinity. The curve, together with the operation, form a finite cyclic group. In simple words, every time we add two points we get a third one which belongs to the curve (it is closed under the operation). We also have an identity point (the point at infinity, $P+\mathcal{O}$) and each point $P$ has an inverse $P^\prime$, such that $P+P^\prime=\mathcal{O}$. Moreover, the elements of the group can be generared by repeteadly adding a point $g$ (the generator) to itself. In other words, for $P$ in the group, there is some $k$ such that $kg=P$. If we are given $k$, we can quickly calculate $P$, but doing the operation the other way around (that is, given $P$, find $k$) can be very difficult (this is known as the discrete logarithm problem) and we used this idea in a previous paragraph. 

All these calculations are done with the operations of the finite field $\mathcal{F}_p$. We see that, at each addition step, we have to calculate the slope of the line, which involves a division on elements of the finite field. This can be rewritten as $s=(x_2-x_1)^{-1}(y_2-y_1)$, where $(x_2-x_1)^{-1}=b$ is the multiplicative inverse of $x_2-x_1$. In simpler form, $b(x_2-x_1)\equiv 1 \pmod{p}$ (When we write $a \equiv b \pmod{p}$, we say that there is some integer $q$ such that a=pq+b. It is read $a$ is congruent to $b$ modulo $p$). Computing inverses is possible, but quite more expensive than multiplications. There is a result from number theory called [Fermat's little theorem](https://en.wikipedia.org/wiki/Fermat%27s_little_theorem), which tells us that $a^{p-1}\equiv 1 \pmod{p}$ if $a$ and $p$ have no common divisors other than 1 (we say $a$ and $p$ are coprime). We can write this in a different fashion,

$a^{p-2}a\equiv 1 \pmod{p}$

and we see that $b=a^{p-2}$ (we can make things simple and then reduce $b$ to $a^{p-2} \pmod{p}$). So, to get the multiplicative inverse, we have to perform many multiplications. (Sometimes it is much easier. Let's take $p=5$ and we try to find $4^{-1}$. We can see that if we do $4\times 4=16 \equiv 1 \pmod{5}$, so $4^{-1}=4$. This is rather strange, but we have to remember that operations on the finite field have a different behavior). As a matter of fact, $p-1$ gives an upper bound to the power $n$ we have to apply to a field element $a$ to get its inverse, that is $a^n \equiv 1 \pmod{p}$. We call the lowest (positive) exponent $n$ such that $a^n \equiv 1 \pmod{p}$ the order of the element. [Lagrange's Theorem](https://en.wikipedia.org/wiki/Lagrange%27s_theorem_(group_theory)) says that the order $n$ divides $p-1$. For example, take $p=7$, so $p-1=6$. We see that $4^3=64\equiv 1 \pmod{7}$, so $4^2\equiv 2 \pmod{7}$ is the inverse of $4$ ($2\times 4=8 \equiv 1 \pmod{7}$). In the same way, $2^3\equiv 1 \pmod{7}$. In the case of $3$, $3^6 \equiv 1 \pmod{7}$ and $3^5 \equiv 5 \pmod{7}$ and we also have $5^6 \equiv 1 \pmod{7}$. So, we see that the orders $n$ are among the divisors of $p-1=6$.

So, even if the equations for point addition over elliptic curves look really simple, they involve many calculations and these may be expensive. If every time we want to add two points, we have to find the multiplicative inverse modulo a large prime, we see that we are paying a high price. There are a couple of tricks we can perform, such as transforming the curve, to gain a lot of speed or avoid some other issues, such as side-channel attacks.

If you are one of those not willing to pay the cost of finding inverses and saving some time or just love speed for the sake of it, then the next section is for you.

## Projective coordinates

We can save ourselves from costly inversions if we move from our nice 2 dimensional space to a 3 dimensional space. This was introduced by Moebius and helps us also to represent the point at infinity properly. We can map our points from our elliptic curve $(x,y)$ to points in projective space $(X,Y,Z)$ as $(x,y) \rightarrow (X=x,Y=y,Z=1)$ and $\mathcal{O} \rightarrow (0,1,0)$. We can go back using the transformation $(X,Y,Z) \rightarrow (x=X/Z,y=Y/Z)$, except for the point at infinity, where it is ill-defined. We can visualize this process with the following picture, where we take three points from an elliptic curve and transform them to 3-d.

![](https://i.imgur.com/zmlMAg9.jpg)


We can think of this as transforming our 2-d points to lines passing through the origin in 3-d space. For example, the point $(x_1,y_1)$ in 2-d transforms to the line $(\mu x_1,\mu y_1, \mu)$ with $\mu$ an element in the field. Thus, two points $P_1=(X_1,Y_1,Z_1)$ and $P_2=(X_2,Y_2,Z_2)$ are the same in 2-d (more precisely, are congruent) if we can find $\eta$ such that $(\eta X_1,\eta Y_1,\eta Z_1)=(X_2,Y_2,Z_2)$. These lines do not contain the origin $(0,0,0)$. It is usual to write points is projective space as $(X:Y:Z)$, instead of $(X,Y,Z)$. In our picture, the point A (yellow) gets mapped to the point D (red above it). All the points that lie on the same straight line passing through the origin and D (pink dashed) are considered equivalent to D. Similarly, point B (blue) is mapped to point F (light blue) and all the ponts over the light green dotted line (except the origin) are equivalent to F. When we add points in this space, the components $(X,Y,Z)$ will change, but we can go back to the point belonging to the curve by just retracing our steps to $Z=1$ along the line that passes through the origin. Why go all this length? We will shortly see that we avoid inversions at each addition step and do just one at the time of finding the point in 2-d (for example, when we need to find $r=x_1$ in ECDSA). Of course, if we have to do $P=2g$ we didn't gain anything, but if we have to perform $P=kg$ with $k$ in the order of 256 bits, we saved many costly inversions.

Making the substitutions into the elliptic curve equation
$$\left(\frac{Y}{Z}\right)^2=\left(\frac{X}{Z}\right)^3+a\left(\frac{X}{Z}\right)+b$$
We can multiply by $Z^3$ and get the equation
$$ZY^2=X^3+aZ^2+bZ^3$$
If we want to sum $P$ and $Q$ to yield $R=P+Q$ in projective space, we can use the formulae:

$Z_R=Z_PZ_Q(X_PZ_Q-X_QZ_P)^3$
$X_R=(X_PZ_Q-X_QZ_P)(Z_QZ_P(Y_PZ_Q-Y_QZ_P)^2-(X_PZ_Q-X_QZ_P)^2(X_PZ_Q+X_QZP))$
$Y_R=Z_PZ_Q(X_QY_P-X_PY_Q)(X_PZ_Q-X_QZ_P)^2-(Y_PZ_Q-Y_QZ_P)A$
$A=Z_PZ_Q(Y_PZ_Q-Y_QZ_P)^2-(X_PZ_Q+X_QZ_P)(X_PZ_Q-X_QZ_P)^2$.

This looks more complicated and difficult than the simple formulae for 2 dimensional (2-d) space. However, we do not have to calculate any inverses! To get the sum, we have to perform 12 multiplications and 2 squarings. In 2-d, we have 2 multiplications, one squaring and one inversion. Inversions can be 20 times or more expensive than multiplications, so we've saved at least 10 multiplications (some authors say inversions are about 80 times more expensive than multiplications).

Some curves can go even faster. If $x^3+ax+b$ has a solution in $\mathcal{F}_p$, we can work with an equivalent Jacobi quartic $v^2=a^\prime u^4+du^2+1$, where $a^\prime$ and $d$ depend on the root. We can transform the curve $(u,v)$ to 3-d space $(U,V,W)$ using $u=U/W$ and $v=V/W^2$ and get the equation

$$V^2=a^\prime U^4+dU^2W^2+W^4$$

If we want to sum $P_3=P_1+P_2$, in these coordinates we have:

$U_3=U_1W_1V_2+U_2W_2V_1$
$V_3=((W_1W_2)^2+a^\prime (U_1U_2)^2)(V_1V_2+dU_1U_2W_1W_2)+2a^\prime U_1U_2W_1W_2(U_1^2W_2^2+U_2^2W_1^2)$
$W_3=(W_1W_2)^2-a^\prime (U_1U_2)^2$

These allow us to further reduce the costs for adding to 6 multiplications and 4 squarings. Other models with fast implementations are Edwards curves and Montgomery curves, which have some of the fastest implementations.

Montgomery curves satisfy the following equation
$$By^2=x^3+Ax^2+x$$
where $B(A^2-4)\neq 0$. This expression can be cast in the Weierstrass form by making some transformation. If we take $(x,y)$ and map it to $(x^\prime,y^\prime)$ given by $(x,y)\rightarrow(x/B+A/3B,y/B)$, we get
$$y^2=x^3+\left(\frac{3-A^2}{3B^2}\right)x+\frac{2A^3-9A}{27B^3}$$
Transforming a Weierstrass curve into a Montgomery curve is not always possible, though. The order of the group must be divisible by $4$ and $x^3+ax+b=0$ must have a solution.

Montgomery curves can also be related to twisted Edwards curves, which obey the following equation
$$ax^2+y^2=1+dx^2y^2$$
The parameters are related via $A=2(a+d)/(a-d)$ and $B=4/(a-d)$. We say these two curves are birrationally equivalent. For example, the well-known Edwards curve 25519, with $p=2^{255}-19$ is (birrationally) equivalent to the Montgomery curve $t^2=u^3+486662u^2+u$. The mappings are
$(x,y)=(\sqrt{-486664}u/t,(u-1)/(u+1))$
$(u,t)=((1+y)/(1-y),\sqrt{-486664}(1+y)/(x(1-y)))$

Montgomery curves have some interesting properties that lend themselves to constant time implementation. We can work in projective coordinates just using the $x$ component, with the transformation $x=X/Z$. Doubling a point takes the simple form:
$4R=(X_1+Z_1)^2-(X_1-Z_1)^2$
$X_2=(X_1+Z_1)^2(X_1-Z_1)^2$
$Z_2=R((X_1-Z_1)^2+((A+2)/4)R)$

Twisted Edwards curves have there advantages, too. The expressions for point addition and doubling are the same. Given $P_1=(x_1,y_1)$, $P_2=(x_2,y_2)$ we get
$x_3=\frac{x_1y_2+x_2y_1}{1+dx_1x_2y_1y_2}$
$y_3=\frac{y_1y_2-ax_1x_2}{1-dx_1x_2y_1y_2}$
If we let $x_1=x_2$ and $y_1=y_2$ we get the expressions for point doubling. There are several alternatives to speeding up the calculations, such as projective, inverted or extended coordinates.

There are some other tricks to add and multiply points over elliptic curves, such as the technique by Gallant, Lambert and Vanstone (GLV) and generalized by Galbraith, Lin and Scott (GLS). 

## Summary

Elliptic curves have gained acceptance in cryptography because they offer good levels of security with short key lengths and allow for faster implementations than other methods such as RSA. This allows smartphones and other less powerful devices to perform cryptographic operations in a fast and reliable way. 

Using the chord-and-tangent method, we can generate finite cyclic groups; in applications, we are generally interested in calculating $kg$, where $k$ is an integer and $g$ is a point in the elliptic curve. The main drawback is that we need to find multiplicative inverses of field elements, which involve many multiplications.

We can improve the speed of these computations by performing transformations between curves (for example, taking a Weierstrass curve to Montgomery form) and using projective coordinates. This way, we avoid calculating multiplicative inverses at each step, at the expense of a few extra multiplications (this extra cost is usually negligible to the overall cost of the inversion). There are also more advanced techniques allowing us to jump from one point to a very distant one, such as with GLS.