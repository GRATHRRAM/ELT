#include <stdio.h>
#include <stdlib.h>

int main()
{
	printf("ELT-Easy_Linux_Terminal : V.prot.1");
	printf("\n Type help to show comands");
	char in;
	while (1)
	{
		scanf("%s", in);

		if (in == "clear")
		{
			clear();
		}
		else if (in == "show.files")
		{
			showfiles();
		}
		else if (in == "make.file")
		{
			makefile();
		}
		else if (in == "make.directory")
		{
			makedirectory();
		}
	}
}