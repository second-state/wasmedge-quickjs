#include <stdio.h>
#include "wasmedge.h"

WasmEdge_Result HostInc(void *Data, WasmEdge_MemoryInstanceContext *MemCxt,
                    const WasmEdge_Value *In, WasmEdge_Value *Out) {
  int32_t Val1 = WasmEdge_ValueGetI32(In[0]);
  printf("Runtime(c)=> host_inc call : %d\n",Val1 + 1);
  Out[0] = WasmEdge_ValueGenI32(Val1 + 1);
  return WasmEdge_Result_Success;
}

// mapping dirs
char* dirs = ".:..\0";

int main(int Argc, const char* Argv[]) {
	/* Create the configure context and add the WASI support. */
	/* This step is not necessary unless you need WASI support. */
	WasmEdge_ConfigureContext *ConfCxt = WasmEdge_ConfigureCreate();
	WasmEdge_ConfigureAddHostRegistration(ConfCxt, WasmEdge_HostRegistration_Wasi);
	/* The configure and store context to the VM creation can be NULL. */
	WasmEdge_VMContext *VMCxt = WasmEdge_VMCreate(ConfCxt, NULL);
	WasmEdge_ImportObjectContext *WasiObject = WasmEdge_VMGetImportModuleContext(VMCxt, WasmEdge_HostRegistration_Wasi);
    WasmEdge_ImportObjectInitWASI(WasiObject,Argv+1,Argc-1,NULL,0,&dirs,1,NULL,0);


    /* Create the import object. */
    WasmEdge_String ExportName = WasmEdge_StringCreateByCString("extern");
    WasmEdge_ImportObjectContext *ImpObj = WasmEdge_ImportObjectCreate(ExportName, NULL);
    enum WasmEdge_ValType ParamList[1] = { WasmEdge_ValType_I32 };
    enum WasmEdge_ValType ReturnList[1] = { WasmEdge_ValType_I32 };
    WasmEdge_FunctionTypeContext *HostFType = WasmEdge_FunctionTypeCreate(ParamList, 1, ReturnList, 1);
    WasmEdge_HostFunctionContext *HostFunc = WasmEdge_HostFunctionCreate(HostFType, HostInc, 0);
    WasmEdge_FunctionTypeDelete(HostFType);
    WasmEdge_String HostFuncName = WasmEdge_StringCreateByCString("host_inc");
    WasmEdge_ImportObjectAddHostFunction(ImpObj, HostFuncName, HostFunc);
    WasmEdge_StringDelete(HostFuncName);

    WasmEdge_VMRegisterModuleFromImport(VMCxt, ImpObj);


	/* The parameters and returns arrays. */
	WasmEdge_Value Params[0];
	WasmEdge_Value Returns[0];
	/* Function name. */
	WasmEdge_String FuncName = WasmEdge_StringCreateByCString("_start");
	/* Run the WASM function from file. */
	WasmEdge_Result Res = WasmEdge_VMRunWasmFromFile(VMCxt, Argv[1], FuncName, Params, 0, Returns, 0);

	if (WasmEdge_ResultOK(Res)) {
		printf("\nRuntime(c)=> OK\n");
	} else {
		printf("\nRuntime(c)=> Error message: %s\n", WasmEdge_ResultGetMessage(Res));
	}

	/* Resources deallocations. */
	WasmEdge_VMDelete(VMCxt);
	WasmEdge_ConfigureDelete(ConfCxt);
	WasmEdge_StringDelete(FuncName);
	return 0;
}