#include <stdio.h>
#include <stdlib.h>

void clear()
{
	system("clear");
}
void showfiles()
{
	system("ls");
	printf("\n Done!");
}
void makefile()
{
	char file_name;
	printf("\n Type a name of new file: ");
	scanf("%s", file_name);
	char s = "touch " + file_name;
	system(s);
	printf("\n Done!");
}
void makedirectory()
{
	char dir;
	printf("\n Type a name of new directory: ");
	scanf("%s", dir);
	char s = "mkdir " + dir;
	system(s);
	printf("\n Done!");
}
void changedirectory()
{
	char todir;
	printf("\n Type where you want go\n");
	printf("\\h: ");
	scanf("%s", todir);
	char s = "cd " + todir;
	system(s);
	printf("\n Done");
}
