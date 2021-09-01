#include <stdio.h>

int32_t host_inc(int32_t x) __attribute__((
__import_module__("extern"),
__import_name__("host_inc")
));

int main(int argc,char** argv){
	printf("hello\n");
	return host_inc(3);
}
