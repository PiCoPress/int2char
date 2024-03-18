#include <stdio.h>
#include <stdlib.h>
#include <string.h>

unsigned short log10p1(long long int u)
{
	unsigned short exp = 1;
	while(u /= 10) exp ++;
	return exp;
}

int main(int argc, char* argv[])
{
	if(argc == 1)
	{
		printf("Usage: int2char [INTEGER]...\n"\
				"\tconvert INTEGER to UTF-8 - negatives (< 0) and larges (>= 2,097,152) will be ignored.\n");
		return 0;
	}
	int i, rpt = 1;
	while(rpt < argc)
	{
		long long int i = atoll(argv[rpt]);
		#ifdef VERIFY_OVERFLOW
		if(log10p1(i) > 17) fprintf(stderr, "EXCEPTION\n");
		else
		#endif
			if(i < 0) { fprintf(stderr, "Ignored negative number: %lld\n", i); }
		else if(i < 256) printf("%c", (int) i);
		else if(i < 2048)
		{
			char k[2] = {192 + (i >> 6), 128 + i % 64};
			printf("%c%c", k[0], k[1]);
		} else if(i < 65536)
		{	
			char k[3] = {
						224 + (i >> 12),
			   			128 + (i >> 6) % 64,
						128 + i % 64
			};
			printf("%c%c%c", k[0], k[1], k[2]);
		} else if(i < 2097152)
		{
			char k[4] = {
						240 + (i >> 18),
						128 + (i >> 12) % 64,
						128 + (i >> 6) % 64,
						128 + i % 64
			};
			printf("%c%c%c%c", k[0], k[1], k[2], k[3]);
		} else { fprintf(stderr, "Ignored large number: %lld\n", i); }
		rpt ++;
	}
	return 0;
}
