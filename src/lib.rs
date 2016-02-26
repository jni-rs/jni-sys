#![allow(non_snake_case, non_camel_case_types)]

extern crate libc;

use std::mem;

// FIXME is this sufficiently correct?
pub type va_list = *mut libc::c_void;

pub type jint = i32;
pub type jlong = i64;
pub type jbyte = i8;
pub type jboolean = u8;
pub type jchar = u16;
pub type jshort = i16;
pub type jfloat = f32;
pub type jdouble = f64;
pub type jsize = jint;

pub struct _jobject(libc::c_void);
pub type jobject = *mut _jobject;
pub type jclass = jobject;
pub type jthrowable = jobject;
pub type jstring = jobject;
pub type jarray = jobject;
pub type jbooleanArray = jarray;
pub type jbyteArray = jarray;
pub type jcharArray = jarray;
pub type jshortArray = jarray;
pub type jintArray = jarray;
pub type jlongArray = jarray;
pub type jfloatArray = jarray;
pub type jdoubleArray = jarray;
pub type jobjectArray = jarray;
pub type jweak = jobject;

#[repr(C)]
#[derive(Copy)]
pub struct jvalue {
    pub _bindgen_data_: [u64; 1],
}

impl jvalue {
    pub unsafe fn z(&mut self) -> *mut jboolean {
        let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn b(&mut self) -> *mut jbyte {
        let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn c(&mut self) -> *mut jchar {
        let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn s(&mut self) -> *mut jshort {
        let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn i(&mut self) -> *mut jint {
        let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn j(&mut self) -> *mut jlong {
        let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn f(&mut self) -> *mut jfloat {
        let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn d(&mut self) -> *mut jdouble {
        let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn l(&mut self) -> *mut jobject {
        let raw: *mut u8 = mem::transmute(&self._bindgen_data_);
        mem::transmute(raw.offset(0))
    }
}

impl Clone for jvalue {
    fn clone(&self) -> Self { *self }
}
impl Default for jvalue {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

pub struct _jfieldID(libc::c_void);
pub type jfieldID = *mut _jfieldID;
pub struct _jmethodID(libc::c_void);
pub type jmethodID = *mut _jmethodID;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum jobjectRefType {
    JNIInvalidRefType = 0,
    JNILocalRefType = 1,
    JNIGlobalRefType = 2,
    JNIWeakGlobalRefType = 3,
}

pub const JNI_FALSE: jboolean = 0;
pub const JNI_TRUE: jboolean = 1;

pub const JNI_OK: jint = 0;
pub const JNI_ERR: jint = -1;
pub const JNI_EDETACHED: jint = -2;
pub const JNI_EVERSION: jint = -3;
pub const JNI_ENOMEM: jint = -4;
pub const JNI_EEXIST: jint = -5;
pub const JNI_EINVAL: jint = -6;

pub const JNI_COMMIT: jint = 1;
pub const JNI_ABORT: jint = 2;

#[repr(C)]
#[derive(Copy)]
pub struct JNINativeMethod {
    pub name: *mut libc::c_char,
    pub signature: *mut libc::c_char,
    pub fnPtr: *mut libc::c_void,
}

impl Clone for JNINativeMethod {
    fn clone(&self) -> Self { *self }
}

impl Default for JNINativeMethod {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

pub type JNIEnv = *const JNINativeInterface_;
pub type JavaVM = *const JNIInvokeInterface_;

#[repr(C)]
#[derive(Copy)]
pub struct JNINativeInterface_ {
    pub reserved0: *mut libc::c_void,
    pub reserved1: *mut libc::c_void,
    pub reserved2: *mut libc::c_void,
    pub reserved3: *mut libc::c_void,
    pub GetVersion: unsafe extern "C" fn(env:
                                                                   *mut JNIEnv)
                                              -> jint,
    pub DefineClass: unsafe extern "C" fn(env:
                                                                    *mut JNIEnv,
                                                                name:
                                                                    *const libc::c_char,
                                                                loader:
                                                                    jobject,
                                                                buf:
                                                                    *const jbyte,
                                                                len: jsize)
                                               -> jclass,
    pub FindClass: unsafe extern "C" fn(env:
                                                                  *mut JNIEnv,
                                                              name:
                                                                  *const libc::c_char)
                                             -> jclass,
    pub FromReflectedMethod: unsafe extern "C" fn(env:
                                                                            *mut JNIEnv,
                                                                        method:
                                                                            jobject)
                                                       -> jmethodID,
    pub FromReflectedField: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       field:
                                                                           jobject)
                                                      -> jfieldID,
    pub ToReflectedMethod: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv,
                                                                      cls:
                                                                          jclass,
                                                                      methodID:
                                                                          jmethodID,
                                                                      isStatic:
                                                                          jboolean)
                                                     -> jobject,
    pub GetSuperclass: unsafe extern "C" fn(env:
                                                                      *mut JNIEnv,
                                                                  sub: jclass)
                                                 -> jclass,
    pub IsAssignableFrom: unsafe extern "C" fn(env:
                                                                         *mut JNIEnv,
                                                                     sub:
                                                                         jclass,
                                                                     sup:
                                                                         jclass)
                                                    -> jboolean,
    pub ToReflectedField: unsafe extern "C" fn(env:
                                                                         *mut JNIEnv,
                                                                     cls:
                                                                         jclass,
                                                                     fieldID:
                                                                         jfieldID,
                                                                     isStatic:
                                                                         jboolean)
                                                    -> jobject,
    pub Throw: unsafe extern "C" fn(env: *mut JNIEnv,
                                                          obj: jthrowable)
                                         -> jint,
    pub ThrowNew: unsafe extern "C" fn(env: *mut JNIEnv,
                                                             clazz: jclass,
                                                             msg:
                                                                 *const libc::c_char)
                                            -> jint,
    pub ExceptionOccurred: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv)
                                                     -> jthrowable,
    pub ExceptionDescribe: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv),
    pub ExceptionClear: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv),
    pub FatalError: unsafe extern "C" fn(env:
                                                                   *mut JNIEnv,
                                                               msg:
                                                                   *const libc::c_char),
    pub PushLocalFrame: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   capacity:
                                                                       jint)
                                                  -> jint,
    pub PopLocalFrame: unsafe extern "C" fn(env:
                                                                      *mut JNIEnv,
                                                                  result:
                                                                      jobject)
                                                 -> jobject,
    pub NewGlobalRef: unsafe extern "C" fn(env:
                                                                     *mut JNIEnv,
                                                                 lobj:
                                                                     jobject)
                                                -> jobject,
    pub DeleteGlobalRef: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    gref:
                                                                        jobject),
    pub DeleteLocalRef: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   obj:
                                                                       jobject),
    pub IsSameObject: unsafe extern "C" fn(env:
                                                                     *mut JNIEnv,
                                                                 obj1:
                                                                     jobject,
                                                                 obj2:
                                                                     jobject)
                                                -> jboolean,
    pub NewLocalRef: unsafe extern "C" fn(env:
                                                                    *mut JNIEnv,
                                                                _ref: jobject)
                                               -> jobject,
    pub EnsureLocalCapacity: unsafe extern "C" fn(env:
                                                                            *mut JNIEnv,
                                                                        capacity:
                                                                            jint)
                                                       -> jint,
    pub AllocObject: unsafe extern "C" fn(env:
                                                                    *mut JNIEnv,
                                                                clazz: jclass)
                                               -> jobject,
    pub NewObject: unsafe extern "C" fn(env:
                                                                  *mut JNIEnv,
                                                              clazz: jclass,
                                                              methodID:
                                                                  jmethodID, ...)
                                             -> jobject,
    pub NewObjectV: unsafe extern "C" fn(env:
                                                                   *mut JNIEnv,
                                                               clazz: jclass,
                                                               methodID:
                                                                   jmethodID,
                                                               args: va_list)
                                              -> jobject,
    pub NewObjectA: unsafe extern "C" fn(env:
                                                                   *mut JNIEnv,
                                                               clazz: jclass,
                                                               methodID:
                                                                   jmethodID,
                                                               args:
                                                                   *const jvalue)
                                              -> jobject,
    pub GetObjectClass: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   obj:
                                                                       jobject)
                                                  -> jclass,
    pub IsInstanceOf: unsafe extern "C" fn(env:
                                                                     *mut JNIEnv,
                                                                 obj: jobject,
                                                                 clazz:
                                                                     jclass)
                                                -> jboolean,
    pub GetMethodID: unsafe extern "C" fn(env:
                                                                    *mut JNIEnv,
                                                                clazz: jclass,
                                                                name:
                                                                    *const libc::c_char,
                                                                sig:
                                                                    *const libc::c_char)
                                               -> jmethodID,
    pub CallObjectMethod: unsafe extern "C" fn(env:
                                                                         *mut JNIEnv,
                                                                     obj:
                                                                         jobject,
                                                                     methodID:
                                                                         jmethodID, ...)
                                                    -> jobject,
    pub CallObjectMethodV: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv,
                                                                      obj:
                                                                          jobject,
                                                                      methodID:
                                                                          jmethodID,
                                                                      args:
                                                                          va_list)
                                                     -> jobject,
    pub CallObjectMethodA: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv,
                                                                      obj:
                                                                          jobject,
                                                                      methodID:
                                                                          jmethodID,
                                                                      args:
                                                                          *const jvalue)
                                                     -> jobject,
    pub CallBooleanMethod: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv,
                                                                      obj:
                                                                          jobject,
                                                                      methodID:
                                                                          jmethodID, ...)
                                                     -> jboolean,
    pub CallBooleanMethodV: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       obj:
                                                                           jobject,
                                                                       methodID:
                                                                           jmethodID,
                                                                       args:
                                                                           va_list)
                                                      -> jboolean,
    pub CallBooleanMethodA: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       obj:
                                                                           jobject,
                                                                       methodID:
                                                                           jmethodID,
                                                                       args:
                                                                           *const jvalue)
                                                      -> jboolean,
    pub CallByteMethod: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   obj:
                                                                       jobject,
                                                                   methodID:
                                                                       jmethodID, ...)
                                                  -> jbyte,
    pub CallByteMethodV: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    obj:
                                                                        jobject,
                                                                    methodID:
                                                                        jmethodID,
                                                                    args:
                                                                        va_list)
                                                   -> jbyte,
    pub CallByteMethodA: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    obj:
                                                                        jobject,
                                                                    methodID:
                                                                        jmethodID,
                                                                    args:
                                                                        *const jvalue)
                                                   -> jbyte,
    pub CallCharMethod: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   obj:
                                                                       jobject,
                                                                   methodID:
                                                                       jmethodID, ...)
                                                  -> jchar,
    pub CallCharMethodV: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    obj:
                                                                        jobject,
                                                                    methodID:
                                                                        jmethodID,
                                                                    args:
                                                                        va_list)
                                                   -> jchar,
    pub CallCharMethodA: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    obj:
                                                                        jobject,
                                                                    methodID:
                                                                        jmethodID,
                                                                    args:
                                                                        *const jvalue)
                                                   -> jchar,
    pub CallShortMethod: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    obj:
                                                                        jobject,
                                                                    methodID:
                                                                        jmethodID, ...)
                                                   -> jshort,
    pub CallShortMethodV: unsafe extern "C" fn(env:
                                                                         *mut JNIEnv,
                                                                     obj:
                                                                         jobject,
                                                                     methodID:
                                                                         jmethodID,
                                                                     args:
                                                                         va_list)
                                                    -> jshort,
    pub CallShortMethodA: unsafe extern "C" fn(env:
                                                                         *mut JNIEnv,
                                                                     obj:
                                                                         jobject,
                                                                     methodID:
                                                                         jmethodID,
                                                                     args:
                                                                         *const jvalue)
                                                    -> jshort,
    pub CallIntMethod: unsafe extern "C" fn(env:
                                                                      *mut JNIEnv,
                                                                  obj:
                                                                      jobject,
                                                                  methodID:
                                                                      jmethodID, ...)
                                                 -> jint,
    pub CallIntMethodV: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   obj:
                                                                       jobject,
                                                                   methodID:
                                                                       jmethodID,
                                                                   args:
                                                                       va_list)
                                                  -> jint,
    pub CallIntMethodA: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   obj:
                                                                       jobject,
                                                                   methodID:
                                                                       jmethodID,
                                                                   args:
                                                                       *const jvalue)
                                                  -> jint,
    pub CallLongMethod: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   obj:
                                                                       jobject,
                                                                   methodID:
                                                                       jmethodID, ...)
                                                  -> jlong,
    pub CallLongMethodV: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    obj:
                                                                        jobject,
                                                                    methodID:
                                                                        jmethodID,
                                                                    args:
                                                                        va_list)
                                                   -> jlong,
    pub CallLongMethodA: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    obj:
                                                                        jobject,
                                                                    methodID:
                                                                        jmethodID,
                                                                    args:
                                                                        *const jvalue)
                                                   -> jlong,
    pub CallFloatMethod: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    obj:
                                                                        jobject,
                                                                    methodID:
                                                                        jmethodID, ...)
                                                   -> jfloat,
    pub CallFloatMethodV: unsafe extern "C" fn(env:
                                                                         *mut JNIEnv,
                                                                     obj:
                                                                         jobject,
                                                                     methodID:
                                                                         jmethodID,
                                                                     args:
                                                                         va_list)
                                                    -> jfloat,
    pub CallFloatMethodA: unsafe extern "C" fn(env:
                                                                         *mut JNIEnv,
                                                                     obj:
                                                                         jobject,
                                                                     methodID:
                                                                         jmethodID,
                                                                     args:
                                                                         *const jvalue)
                                                    -> jfloat,
    pub CallDoubleMethod: unsafe extern "C" fn(env:
                                                                         *mut JNIEnv,
                                                                     obj:
                                                                         jobject,
                                                                     methodID:
                                                                         jmethodID, ...)
                                                    -> jdouble,
    pub CallDoubleMethodV: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv,
                                                                      obj:
                                                                          jobject,
                                                                      methodID:
                                                                          jmethodID,
                                                                      args:
                                                                          va_list)
                                                     -> jdouble,
    pub CallDoubleMethodA: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv,
                                                                      obj:
                                                                          jobject,
                                                                      methodID:
                                                                          jmethodID,
                                                                      args:
                                                                          *const jvalue)
                                                     -> jdouble,
    pub CallVoidMethod: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   obj:
                                                                       jobject,
                                                                   methodID:
                                                                       jmethodID, ...),
    pub CallVoidMethodV: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    obj:
                                                                        jobject,
                                                                    methodID:
                                                                        jmethodID,
                                                                    args:
                                                                        va_list),
    pub CallVoidMethodA: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    obj:
                                                                        jobject,
                                                                    methodID:
                                                                        jmethodID,
                                                                    args:
                                                                        *const jvalue),
    pub CallNonvirtualObjectMethod: unsafe extern "C" fn(env:
                                                                                   *mut JNIEnv,
                                                                               obj:
                                                                                   jobject,
                                                                               clazz:
                                                                                   jclass,
                                                                               methodID:
                                                                                   jmethodID, ...)
                                                              -> jobject,
    pub CallNonvirtualObjectMethodV: unsafe extern "C" fn(env:
                                                                                    *mut JNIEnv,
                                                                                obj:
                                                                                    jobject,
                                                                                clazz:
                                                                                    jclass,
                                                                                methodID:
                                                                                    jmethodID,
                                                                                args:
                                                                                    va_list)
                                                               -> jobject,
    pub CallNonvirtualObjectMethodA: unsafe extern "C" fn(env:
                                                                                    *mut JNIEnv,
                                                                                obj:
                                                                                    jobject,
                                                                                clazz:
                                                                                    jclass,
                                                                                methodID:
                                                                                    jmethodID,
                                                                                args:
                                                                                    *const jvalue)
                                                               -> jobject,
    pub CallNonvirtualBooleanMethod: unsafe extern "C" fn(env:
                                                                                    *mut JNIEnv,
                                                                                obj:
                                                                                    jobject,
                                                                                clazz:
                                                                                    jclass,
                                                                                methodID:
                                                                                    jmethodID, ...)
                                                               -> jboolean,
    pub CallNonvirtualBooleanMethodV: unsafe extern "C" fn(env:
                                                                                     *mut JNIEnv,
                                                                                 obj:
                                                                                     jobject,
                                                                                 clazz:
                                                                                     jclass,
                                                                                 methodID:
                                                                                     jmethodID,
                                                                                 args:
                                                                                     va_list)
                                                                -> jboolean,
    pub CallNonvirtualBooleanMethodA: unsafe extern "C" fn(env:
                                                                                     *mut JNIEnv,
                                                                                 obj:
                                                                                     jobject,
                                                                                 clazz:
                                                                                     jclass,
                                                                                 methodID:
                                                                                     jmethodID,
                                                                                 args:
                                                                                     *const jvalue)
                                                                -> jboolean,
    pub CallNonvirtualByteMethod: unsafe extern "C" fn(env:
                                                                                 *mut JNIEnv,
                                                                             obj:
                                                                                 jobject,
                                                                             clazz:
                                                                                 jclass,
                                                                             methodID:
                                                                                 jmethodID, ...)
                                                            -> jbyte,
    pub CallNonvirtualByteMethodV: unsafe extern "C" fn(env:
                                                                                  *mut JNIEnv,
                                                                              obj:
                                                                                  jobject,
                                                                              clazz:
                                                                                  jclass,
                                                                              methodID:
                                                                                  jmethodID,
                                                                              args:
                                                                                  va_list)
                                                             -> jbyte,
    pub CallNonvirtualByteMethodA: unsafe extern "C" fn(env:
                                                                                  *mut JNIEnv,
                                                                              obj:
                                                                                  jobject,
                                                                              clazz:
                                                                                  jclass,
                                                                              methodID:
                                                                                  jmethodID,
                                                                              args:
                                                                                  *const jvalue)
                                                             -> jbyte,
    pub CallNonvirtualCharMethod: unsafe extern "C" fn(env:
                                                                                 *mut JNIEnv,
                                                                             obj:
                                                                                 jobject,
                                                                             clazz:
                                                                                 jclass,
                                                                             methodID:
                                                                                 jmethodID, ...)
                                                            -> jchar,
    pub CallNonvirtualCharMethodV: unsafe extern "C" fn(env:
                                                                                  *mut JNIEnv,
                                                                              obj:
                                                                                  jobject,
                                                                              clazz:
                                                                                  jclass,
                                                                              methodID:
                                                                                  jmethodID,
                                                                              args:
                                                                                  va_list)
                                                             -> jchar,
    pub CallNonvirtualCharMethodA: unsafe extern "C" fn(env:
                                                                                  *mut JNIEnv,
                                                                              obj:
                                                                                  jobject,
                                                                              clazz:
                                                                                  jclass,
                                                                              methodID:
                                                                                  jmethodID,
                                                                              args:
                                                                                  *const jvalue)
                                                             -> jchar,
    pub CallNonvirtualShortMethod: unsafe extern "C" fn(env:
                                                                                  *mut JNIEnv,
                                                                              obj:
                                                                                  jobject,
                                                                              clazz:
                                                                                  jclass,
                                                                              methodID:
                                                                                  jmethodID, ...)
                                                             -> jshort,
    pub CallNonvirtualShortMethodV: unsafe extern "C" fn(env:
                                                                                   *mut JNIEnv,
                                                                               obj:
                                                                                   jobject,
                                                                               clazz:
                                                                                   jclass,
                                                                               methodID:
                                                                                   jmethodID,
                                                                               args:
                                                                                   va_list)
                                                              -> jshort,
    pub CallNonvirtualShortMethodA: unsafe extern "C" fn(env:
                                                                                   *mut JNIEnv,
                                                                               obj:
                                                                                   jobject,
                                                                               clazz:
                                                                                   jclass,
                                                                               methodID:
                                                                                   jmethodID,
                                                                               args:
                                                                                   *const jvalue)
                                                              -> jshort,
    pub CallNonvirtualIntMethod: unsafe extern "C" fn(env:
                                                                                *mut JNIEnv,
                                                                            obj:
                                                                                jobject,
                                                                            clazz:
                                                                                jclass,
                                                                            methodID:
                                                                                jmethodID, ...)
                                                           -> jint,
    pub CallNonvirtualIntMethodV: unsafe extern "C" fn(env:
                                                                                 *mut JNIEnv,
                                                                             obj:
                                                                                 jobject,
                                                                             clazz:
                                                                                 jclass,
                                                                             methodID:
                                                                                 jmethodID,
                                                                             args:
                                                                                 va_list)
                                                            -> jint,
    pub CallNonvirtualIntMethodA: unsafe extern "C" fn(env:
                                                                                 *mut JNIEnv,
                                                                             obj:
                                                                                 jobject,
                                                                             clazz:
                                                                                 jclass,
                                                                             methodID:
                                                                                 jmethodID,
                                                                             args:
                                                                                 *const jvalue)
                                                            -> jint,
    pub CallNonvirtualLongMethod: unsafe extern "C" fn(env:
                                                                                 *mut JNIEnv,
                                                                             obj:
                                                                                 jobject,
                                                                             clazz:
                                                                                 jclass,
                                                                             methodID:
                                                                                 jmethodID, ...)
                                                            -> jlong,
    pub CallNonvirtualLongMethodV: unsafe extern "C" fn(env:
                                                                                  *mut JNIEnv,
                                                                              obj:
                                                                                  jobject,
                                                                              clazz:
                                                                                  jclass,
                                                                              methodID:
                                                                                  jmethodID,
                                                                              args:
                                                                                  va_list)
                                                             -> jlong,
    pub CallNonvirtualLongMethodA: unsafe extern "C" fn(env:
                                                                                  *mut JNIEnv,
                                                                              obj:
                                                                                  jobject,
                                                                              clazz:
                                                                                  jclass,
                                                                              methodID:
                                                                                  jmethodID,
                                                                              args:
                                                                                  *const jvalue)
                                                             -> jlong,
    pub CallNonvirtualFloatMethod: unsafe extern "C" fn(env:
                                                                                  *mut JNIEnv,
                                                                              obj:
                                                                                  jobject,
                                                                              clazz:
                                                                                  jclass,
                                                                              methodID:
                                                                                  jmethodID, ...)
                                                             -> jfloat,
    pub CallNonvirtualFloatMethodV: unsafe extern "C" fn(env:
                                                                                   *mut JNIEnv,
                                                                               obj:
                                                                                   jobject,
                                                                               clazz:
                                                                                   jclass,
                                                                               methodID:
                                                                                   jmethodID,
                                                                               args:
                                                                                   va_list)
                                                              -> jfloat,
    pub CallNonvirtualFloatMethodA: unsafe extern "C" fn(env:
                                                                                   *mut JNIEnv,
                                                                               obj:
                                                                                   jobject,
                                                                               clazz:
                                                                                   jclass,
                                                                               methodID:
                                                                                   jmethodID,
                                                                               args:
                                                                                   *const jvalue)
                                                              -> jfloat,
    pub CallNonvirtualDoubleMethod: unsafe extern "C" fn(env:
                                                                                   *mut JNIEnv,
                                                                               obj:
                                                                                   jobject,
                                                                               clazz:
                                                                                   jclass,
                                                                               methodID:
                                                                                   jmethodID, ...)
                                                              -> jdouble,
    pub CallNonvirtualDoubleMethodV: unsafe extern "C" fn(env:
                                                                                    *mut JNIEnv,
                                                                                obj:
                                                                                    jobject,
                                                                                clazz:
                                                                                    jclass,
                                                                                methodID:
                                                                                    jmethodID,
                                                                                args:
                                                                                    va_list)
                                                               -> jdouble,
    pub CallNonvirtualDoubleMethodA: unsafe extern "C" fn(env:
                                                                                    *mut JNIEnv,
                                                                                obj:
                                                                                    jobject,
                                                                                clazz:
                                                                                    jclass,
                                                                                methodID:
                                                                                    jmethodID,
                                                                                args:
                                                                                    *const jvalue)
                                                               -> jdouble,
    pub CallNonvirtualVoidMethod: unsafe extern "C" fn(env:
                                                                                 *mut JNIEnv,
                                                                             obj:
                                                                                 jobject,
                                                                             clazz:
                                                                                 jclass,
                                                                             methodID:
                                                                                 jmethodID, ...),
    pub CallNonvirtualVoidMethodV: unsafe extern "C" fn(env:
                                                                                  *mut JNIEnv,
                                                                              obj:
                                                                                  jobject,
                                                                              clazz:
                                                                                  jclass,
                                                                              methodID:
                                                                                  jmethodID,
                                                                              args:
                                                                                  va_list),
    pub CallNonvirtualVoidMethodA: unsafe extern "C" fn(env:
                                                                                  *mut JNIEnv,
                                                                              obj:
                                                                                  jobject,
                                                                              clazz:
                                                                                  jclass,
                                                                              methodID:
                                                                                  jmethodID,
                                                                              args:
                                                                                  *const jvalue),
    pub GetFieldID: unsafe extern "C" fn(env:
                                                                   *mut JNIEnv,
                                                               clazz: jclass,
                                                               name:
                                                                   *const libc::c_char,
                                                               sig:
                                                                   *const libc::c_char)
                                              -> jfieldID,
    pub GetObjectField: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   obj:
                                                                       jobject,
                                                                   fieldID:
                                                                       jfieldID)
                                                  -> jobject,
    pub GetBooleanField: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    obj:
                                                                        jobject,
                                                                    fieldID:
                                                                        jfieldID)
                                                   -> jboolean,
    pub GetByteField: unsafe extern "C" fn(env:
                                                                     *mut JNIEnv,
                                                                 obj: jobject,
                                                                 fieldID:
                                                                     jfieldID)
                                                -> jbyte,
    pub GetCharField: unsafe extern "C" fn(env:
                                                                     *mut JNIEnv,
                                                                 obj: jobject,
                                                                 fieldID:
                                                                     jfieldID)
                                                -> jchar,
    pub GetShortField: unsafe extern "C" fn(env:
                                                                      *mut JNIEnv,
                                                                  obj:
                                                                      jobject,
                                                                  fieldID:
                                                                      jfieldID)
                                                 -> jshort,
    pub GetIntField: unsafe extern "C" fn(env:
                                                                    *mut JNIEnv,
                                                                obj: jobject,
                                                                fieldID:
                                                                    jfieldID)
                                               -> jint,
    pub GetLongField: unsafe extern "C" fn(env:
                                                                     *mut JNIEnv,
                                                                 obj: jobject,
                                                                 fieldID:
                                                                     jfieldID)
                                                -> jlong,
    pub GetFloatField: unsafe extern "C" fn(env:
                                                                      *mut JNIEnv,
                                                                  obj:
                                                                      jobject,
                                                                  fieldID:
                                                                      jfieldID)
                                                 -> jfloat,
    pub GetDoubleField: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   obj:
                                                                       jobject,
                                                                   fieldID:
                                                                       jfieldID)
                                                  -> jdouble,
    pub SetObjectField: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   obj:
                                                                       jobject,
                                                                   fieldID:
                                                                       jfieldID,
                                                                   val:
                                                                       jobject),
    pub SetBooleanField: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    obj:
                                                                        jobject,
                                                                    fieldID:
                                                                        jfieldID,
                                                                    val:
                                                                        jboolean),
    pub SetByteField: unsafe extern "C" fn(env:
                                                                     *mut JNIEnv,
                                                                 obj: jobject,
                                                                 fieldID:
                                                                     jfieldID,
                                                                 val: jbyte),
    pub SetCharField: unsafe extern "C" fn(env:
                                                                     *mut JNIEnv,
                                                                 obj: jobject,
                                                                 fieldID:
                                                                     jfieldID,
                                                                 val: jchar),
    pub SetShortField: unsafe extern "C" fn(env:
                                                                      *mut JNIEnv,
                                                                  obj:
                                                                      jobject,
                                                                  fieldID:
                                                                      jfieldID,
                                                                  val:
                                                                      jshort),
    pub SetIntField: unsafe extern "C" fn(env:
                                                                    *mut JNIEnv,
                                                                obj: jobject,
                                                                fieldID:
                                                                    jfieldID,
                                                                val: jint),
    pub SetLongField: unsafe extern "C" fn(env:
                                                                     *mut JNIEnv,
                                                                 obj: jobject,
                                                                 fieldID:
                                                                     jfieldID,
                                                                 val: jlong),
    pub SetFloatField: unsafe extern "C" fn(env:
                                                                      *mut JNIEnv,
                                                                  obj:
                                                                      jobject,
                                                                  fieldID:
                                                                      jfieldID,
                                                                  val:
                                                                      jfloat),
    pub SetDoubleField: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   obj:
                                                                       jobject,
                                                                   fieldID:
                                                                       jfieldID,
                                                                   val:
                                                                       jdouble),
    pub GetStaticMethodID: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv,
                                                                      clazz:
                                                                          jclass,
                                                                      name:
                                                                          *const libc::c_char,
                                                                      sig:
                                                                          *const libc::c_char)
                                                     -> jmethodID,
    pub CallStaticObjectMethod: unsafe extern "C" fn(env:
                                                                               *mut JNIEnv,
                                                                           clazz:
                                                                               jclass,
                                                                           methodID:
                                                                               jmethodID, ...)
                                                          -> jobject,
    pub CallStaticObjectMethodV: unsafe extern "C" fn(env:
                                                                                *mut JNIEnv,
                                                                            clazz:
                                                                                jclass,
                                                                            methodID:
                                                                                jmethodID,
                                                                            args:
                                                                                va_list)
                                                           -> jobject,
    pub CallStaticObjectMethodA: unsafe extern "C" fn(env:
                                                                                *mut JNIEnv,
                                                                            clazz:
                                                                                jclass,
                                                                            methodID:
                                                                                jmethodID,
                                                                            args:
                                                                                *const jvalue)
                                                           -> jobject,
    pub CallStaticBooleanMethod: unsafe extern "C" fn(env:
                                                                                *mut JNIEnv,
                                                                            clazz:
                                                                                jclass,
                                                                            methodID:
                                                                                jmethodID, ...)
                                                           -> jboolean,
    pub CallStaticBooleanMethodV: unsafe extern "C" fn(env:
                                                                                 *mut JNIEnv,
                                                                             clazz:
                                                                                 jclass,
                                                                             methodID:
                                                                                 jmethodID,
                                                                             args:
                                                                                 va_list)
                                                            -> jboolean,
    pub CallStaticBooleanMethodA: unsafe extern "C" fn(env:
                                                                                 *mut JNIEnv,
                                                                             clazz:
                                                                                 jclass,
                                                                             methodID:
                                                                                 jmethodID,
                                                                             args:
                                                                                 *const jvalue)
                                                            -> jboolean,
    pub CallStaticByteMethod: unsafe extern "C" fn(env:
                                                                             *mut JNIEnv,
                                                                         clazz:
                                                                             jclass,
                                                                         methodID:
                                                                             jmethodID, ...)
                                                        -> jbyte,
    pub CallStaticByteMethodV: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          clazz:
                                                                              jclass,
                                                                          methodID:
                                                                              jmethodID,
                                                                          args:
                                                                              va_list)
                                                         -> jbyte,
    pub CallStaticByteMethodA: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          clazz:
                                                                              jclass,
                                                                          methodID:
                                                                              jmethodID,
                                                                          args:
                                                                              *const jvalue)
                                                         -> jbyte,
    pub CallStaticCharMethod: unsafe extern "C" fn(env:
                                                                             *mut JNIEnv,
                                                                         clazz:
                                                                             jclass,
                                                                         methodID:
                                                                             jmethodID, ...)
                                                        -> jchar,
    pub CallStaticCharMethodV: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          clazz:
                                                                              jclass,
                                                                          methodID:
                                                                              jmethodID,
                                                                          args:
                                                                              va_list)
                                                         -> jchar,
    pub CallStaticCharMethodA: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          clazz:
                                                                              jclass,
                                                                          methodID:
                                                                              jmethodID,
                                                                          args:
                                                                              *const jvalue)
                                                         -> jchar,
    pub CallStaticShortMethod: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          clazz:
                                                                              jclass,
                                                                          methodID:
                                                                              jmethodID, ...)
                                                         -> jshort,
    pub CallStaticShortMethodV: unsafe extern "C" fn(env:
                                                                               *mut JNIEnv,
                                                                           clazz:
                                                                               jclass,
                                                                           methodID:
                                                                               jmethodID,
                                                                           args:
                                                                               va_list)
                                                          -> jshort,
    pub CallStaticShortMethodA: unsafe extern "C" fn(env:
                                                                               *mut JNIEnv,
                                                                           clazz:
                                                                               jclass,
                                                                           methodID:
                                                                               jmethodID,
                                                                           args:
                                                                               *const jvalue)
                                                          -> jshort,
    pub CallStaticIntMethod: unsafe extern "C" fn(env:
                                                                            *mut JNIEnv,
                                                                        clazz:
                                                                            jclass,
                                                                        methodID:
                                                                            jmethodID, ...)
                                                       -> jint,
    pub CallStaticIntMethodV: unsafe extern "C" fn(env:
                                                                             *mut JNIEnv,
                                                                         clazz:
                                                                             jclass,
                                                                         methodID:
                                                                             jmethodID,
                                                                         args:
                                                                             va_list)
                                                        -> jint,
    pub CallStaticIntMethodA: unsafe extern "C" fn(env:
                                                                             *mut JNIEnv,
                                                                         clazz:
                                                                             jclass,
                                                                         methodID:
                                                                             jmethodID,
                                                                         args:
                                                                             *const jvalue)
                                                        -> jint,
    pub CallStaticLongMethod: unsafe extern "C" fn(env:
                                                                             *mut JNIEnv,
                                                                         clazz:
                                                                             jclass,
                                                                         methodID:
                                                                             jmethodID, ...)
                                                        -> jlong,
    pub CallStaticLongMethodV: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          clazz:
                                                                              jclass,
                                                                          methodID:
                                                                              jmethodID,
                                                                          args:
                                                                              va_list)
                                                         -> jlong,
    pub CallStaticLongMethodA: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          clazz:
                                                                              jclass,
                                                                          methodID:
                                                                              jmethodID,
                                                                          args:
                                                                              *const jvalue)
                                                         -> jlong,
    pub CallStaticFloatMethod: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          clazz:
                                                                              jclass,
                                                                          methodID:
                                                                              jmethodID, ...)
                                                         -> jfloat,
    pub CallStaticFloatMethodV: unsafe extern "C" fn(env:
                                                                               *mut JNIEnv,
                                                                           clazz:
                                                                               jclass,
                                                                           methodID:
                                                                               jmethodID,
                                                                           args:
                                                                               va_list)
                                                          -> jfloat,
    pub CallStaticFloatMethodA: unsafe extern "C" fn(env:
                                                                               *mut JNIEnv,
                                                                           clazz:
                                                                               jclass,
                                                                           methodID:
                                                                               jmethodID,
                                                                           args:
                                                                               *const jvalue)
                                                          -> jfloat,
    pub CallStaticDoubleMethod: unsafe extern "C" fn(env:
                                                                               *mut JNIEnv,
                                                                           clazz:
                                                                               jclass,
                                                                           methodID:
                                                                               jmethodID, ...)
                                                          -> jdouble,
    pub CallStaticDoubleMethodV: unsafe extern "C" fn(env:
                                                                                *mut JNIEnv,
                                                                            clazz:
                                                                                jclass,
                                                                            methodID:
                                                                                jmethodID,
                                                                            args:
                                                                                va_list)
                                                           -> jdouble,
    pub CallStaticDoubleMethodA: unsafe extern "C" fn(env:
                                                                                *mut JNIEnv,
                                                                            clazz:
                                                                                jclass,
                                                                            methodID:
                                                                                jmethodID,
                                                                            args:
                                                                                *const jvalue)
                                                           -> jdouble,
    pub CallStaticVoidMethod: unsafe extern "C" fn(env:
                                                                             *mut JNIEnv,
                                                                         cls:
                                                                             jclass,
                                                                         methodID:
                                                                             jmethodID, ...),
    pub CallStaticVoidMethodV: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          cls:
                                                                              jclass,
                                                                          methodID:
                                                                              jmethodID,
                                                                          args:
                                                                              va_list),
    pub CallStaticVoidMethodA: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          cls:
                                                                              jclass,
                                                                          methodID:
                                                                              jmethodID,
                                                                          args:
                                                                              *const jvalue),
    pub GetStaticFieldID: unsafe extern "C" fn(env:
                                                                         *mut JNIEnv,
                                                                     clazz:
                                                                         jclass,
                                                                     name:
                                                                         *const libc::c_char,
                                                                     sig:
                                                                         *const libc::c_char)
                                                    -> jfieldID,
    pub GetStaticObjectField: unsafe extern "C" fn(env:
                                                                             *mut JNIEnv,
                                                                         clazz:
                                                                             jclass,
                                                                         fieldID:
                                                                             jfieldID)
                                                        -> jobject,
    pub GetStaticBooleanField: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          clazz:
                                                                              jclass,
                                                                          fieldID:
                                                                              jfieldID)
                                                         -> jboolean,
    pub GetStaticByteField: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       clazz:
                                                                           jclass,
                                                                       fieldID:
                                                                           jfieldID)
                                                      -> jbyte,
    pub GetStaticCharField: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       clazz:
                                                                           jclass,
                                                                       fieldID:
                                                                           jfieldID)
                                                      -> jchar,
    pub GetStaticShortField: unsafe extern "C" fn(env:
                                                                            *mut JNIEnv,
                                                                        clazz:
                                                                            jclass,
                                                                        fieldID:
                                                                            jfieldID)
                                                       -> jshort,
    pub GetStaticIntField: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv,
                                                                      clazz:
                                                                          jclass,
                                                                      fieldID:
                                                                          jfieldID)
                                                     -> jint,
    pub GetStaticLongField: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       clazz:
                                                                           jclass,
                                                                       fieldID:
                                                                           jfieldID)
                                                      -> jlong,
    pub GetStaticFloatField: unsafe extern "C" fn(env:
                                                                            *mut JNIEnv,
                                                                        clazz:
                                                                            jclass,
                                                                        fieldID:
                                                                            jfieldID)
                                                       -> jfloat,
    pub GetStaticDoubleField: unsafe extern "C" fn(env:
                                                                             *mut JNIEnv,
                                                                         clazz:
                                                                             jclass,
                                                                         fieldID:
                                                                             jfieldID)
                                                        -> jdouble,
    pub SetStaticObjectField: unsafe extern "C" fn(env:
                                                                             *mut JNIEnv,
                                                                         clazz:
                                                                             jclass,
                                                                         fieldID:
                                                                             jfieldID,
                                                                         value:
                                                                             jobject),
    pub SetStaticBooleanField: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          clazz:
                                                                              jclass,
                                                                          fieldID:
                                                                              jfieldID,
                                                                          value:
                                                                              jboolean),
    pub SetStaticByteField: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       clazz:
                                                                           jclass,
                                                                       fieldID:
                                                                           jfieldID,
                                                                       value:
                                                                           jbyte),
    pub SetStaticCharField: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       clazz:
                                                                           jclass,
                                                                       fieldID:
                                                                           jfieldID,
                                                                       value:
                                                                           jchar),
    pub SetStaticShortField: unsafe extern "C" fn(env:
                                                                            *mut JNIEnv,
                                                                        clazz:
                                                                            jclass,
                                                                        fieldID:
                                                                            jfieldID,
                                                                        value:
                                                                            jshort),
    pub SetStaticIntField: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv,
                                                                      clazz:
                                                                          jclass,
                                                                      fieldID:
                                                                          jfieldID,
                                                                      value:
                                                                          jint),
    pub SetStaticLongField: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       clazz:
                                                                           jclass,
                                                                       fieldID:
                                                                           jfieldID,
                                                                       value:
                                                                           jlong),
    pub SetStaticFloatField: unsafe extern "C" fn(env:
                                                                            *mut JNIEnv,
                                                                        clazz:
                                                                            jclass,
                                                                        fieldID:
                                                                            jfieldID,
                                                                        value:
                                                                            jfloat),
    pub SetStaticDoubleField: unsafe extern "C" fn(env:
                                                                             *mut JNIEnv,
                                                                         clazz:
                                                                             jclass,
                                                                         fieldID:
                                                                             jfieldID,
                                                                         value:
                                                                             jdouble),
    pub NewString: unsafe extern "C" fn(env:
                                                                  *mut JNIEnv,
                                                              unicode:
                                                                  *const jchar,
                                                              len: jsize)
                                             -> jstring,
    pub GetStringLength: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    str:
                                                                        jstring)
                                                   -> jsize,
    pub GetStringChars: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   str:
                                                                       jstring,
                                                                   isCopy:
                                                                       *mut jboolean)
                                                  -> *const jchar,
    pub ReleaseStringChars: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       str:
                                                                           jstring,
                                                                       chars:
                                                                           *const jchar),
    pub NewStringUTF: unsafe extern "C" fn(env:
                                                                     *mut JNIEnv,
                                                                 utf:
                                                                     *const libc::c_char)
                                                -> jstring,
    pub GetStringUTFLength: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       str:
                                                                           jstring)
                                                      -> jsize,
    pub GetStringUTFChars: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv,
                                                                      str:
                                                                          jstring,
                                                                      isCopy:
                                                                          *mut jboolean)
                                                     ->
                                                         *const libc::c_char,
    pub ReleaseStringUTFChars: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          str:
                                                                              jstring,
                                                                          chars:
                                                                              *const libc::c_char),
    pub GetArrayLength: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   array:
                                                                       jarray)
                                                  -> jsize,
    pub NewObjectArray: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   len: jsize,
                                                                   clazz:
                                                                       jclass,
                                                                   init:
                                                                       jobject)
                                                  -> jobjectArray,
    pub GetObjectArrayElement: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          array:
                                                                              jobjectArray,
                                                                          index:
                                                                              jsize)
                                                         -> jobject,
    pub SetObjectArrayElement: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          array:
                                                                              jobjectArray,
                                                                          index:
                                                                              jsize,
                                                                          val:
                                                                              jobject),
    pub NewBooleanArray: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    len:
                                                                        jsize)
                                                   -> jbooleanArray,
    pub NewByteArray: unsafe extern "C" fn(env:
                                                                     *mut JNIEnv,
                                                                 len: jsize)
                                                -> jbyteArray,
    pub NewCharArray: unsafe extern "C" fn(env:
                                                                     *mut JNIEnv,
                                                                 len: jsize)
                                                -> jcharArray,
    pub NewShortArray: unsafe extern "C" fn(env:
                                                                      *mut JNIEnv,
                                                                  len: jsize)
                                                 -> jshortArray,
    pub NewIntArray: unsafe extern "C" fn(env:
                                                                    *mut JNIEnv,
                                                                len: jsize)
                                               -> jintArray,
    pub NewLongArray: unsafe extern "C" fn(env:
                                                                     *mut JNIEnv,
                                                                 len: jsize)
                                                -> jlongArray,
    pub NewFloatArray: unsafe extern "C" fn(env:
                                                                      *mut JNIEnv,
                                                                  len: jsize)
                                                 -> jfloatArray,
    pub NewDoubleArray: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv,
                                                                   len: jsize)
                                                  -> jdoubleArray,
    pub GetBooleanArrayElements: unsafe extern "C" fn(env:
                                                                                *mut JNIEnv,
                                                                            array:
                                                                                jbooleanArray,
                                                                            isCopy:
                                                                                *mut jboolean)
                                                           -> *mut jboolean,
    pub GetByteArrayElements: unsafe extern "C" fn(env:
                                                                             *mut JNIEnv,
                                                                         array:
                                                                             jbyteArray,
                                                                         isCopy:
                                                                             *mut jboolean)
                                                        -> *mut jbyte,
    pub GetCharArrayElements: unsafe extern "C" fn(env:
                                                                             *mut JNIEnv,
                                                                         array:
                                                                             jcharArray,
                                                                         isCopy:
                                                                             *mut jboolean)
                                                        -> *mut jchar,
    pub GetShortArrayElements: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          array:
                                                                              jshortArray,
                                                                          isCopy:
                                                                              *mut jboolean)
                                                         -> *mut jshort,
    pub GetIntArrayElements: unsafe extern "C" fn(env:
                                                                            *mut JNIEnv,
                                                                        array:
                                                                            jintArray,
                                                                        isCopy:
                                                                            *mut jboolean)
                                                       -> *mut jint,
    pub GetLongArrayElements: unsafe extern "C" fn(env:
                                                                             *mut JNIEnv,
                                                                         array:
                                                                             jlongArray,
                                                                         isCopy:
                                                                             *mut jboolean)
                                                        -> *mut jlong,
    pub GetFloatArrayElements: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          array:
                                                                              jfloatArray,
                                                                          isCopy:
                                                                              *mut jboolean)
                                                         -> *mut jfloat,
    pub GetDoubleArrayElements: unsafe extern "C" fn(env:
                                                                               *mut JNIEnv,
                                                                           array:
                                                                               jdoubleArray,
                                                                           isCopy:
                                                                               *mut jboolean)
                                                          -> *mut jdouble,
    pub ReleaseBooleanArrayElements: unsafe extern "C" fn(env:
                                                                                    *mut JNIEnv,
                                                                                array:
                                                                                    jbooleanArray,
                                                                                elems:
                                                                                    *mut jboolean,
                                                                                mode:
                                                                                    jint),
    pub ReleaseByteArrayElements: unsafe extern "C" fn(env:
                                                                                 *mut JNIEnv,
                                                                             array:
                                                                                 jbyteArray,
                                                                             elems:
                                                                                 *mut jbyte,
                                                                             mode:
                                                                                 jint),
    pub ReleaseCharArrayElements: unsafe extern "C" fn(env:
                                                                                 *mut JNIEnv,
                                                                             array:
                                                                                 jcharArray,
                                                                             elems:
                                                                                 *mut jchar,
                                                                             mode:
                                                                                 jint),
    pub ReleaseShortArrayElements: unsafe extern "C" fn(env:
                                                                                  *mut JNIEnv,
                                                                              array:
                                                                                  jshortArray,
                                                                              elems:
                                                                                  *mut jshort,
                                                                              mode:
                                                                                  jint),
    pub ReleaseIntArrayElements: unsafe extern "C" fn(env:
                                                                                *mut JNIEnv,
                                                                            array:
                                                                                jintArray,
                                                                            elems:
                                                                                *mut jint,
                                                                            mode:
                                                                                jint),
    pub ReleaseLongArrayElements: unsafe extern "C" fn(env:
                                                                                 *mut JNIEnv,
                                                                             array:
                                                                                 jlongArray,
                                                                             elems:
                                                                                 *mut jlong,
                                                                             mode:
                                                                                 jint),
    pub ReleaseFloatArrayElements: unsafe extern "C" fn(env:
                                                                                  *mut JNIEnv,
                                                                              array:
                                                                                  jfloatArray,
                                                                              elems:
                                                                                  *mut jfloat,
                                                                              mode:
                                                                                  jint),
    pub ReleaseDoubleArrayElements: unsafe extern "C" fn(env:
                                                                                   *mut JNIEnv,
                                                                               array:
                                                                                   jdoubleArray,
                                                                               elems:
                                                                                   *mut jdouble,
                                                                               mode:
                                                                                   jint),
    pub GetBooleanArrayRegion: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          array:
                                                                              jbooleanArray,
                                                                          start:
                                                                              jsize,
                                                                          l:
                                                                              jsize,
                                                                          buf:
                                                                              *mut jboolean),
    pub GetByteArrayRegion: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       array:
                                                                           jbyteArray,
                                                                       start:
                                                                           jsize,
                                                                       len:
                                                                           jsize,
                                                                       buf:
                                                                           *mut jbyte),
    pub GetCharArrayRegion: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       array:
                                                                           jcharArray,
                                                                       start:
                                                                           jsize,
                                                                       len:
                                                                           jsize,
                                                                       buf:
                                                                           *mut jchar),
    pub GetShortArrayRegion: unsafe extern "C" fn(env:
                                                                            *mut JNIEnv,
                                                                        array:
                                                                            jshortArray,
                                                                        start:
                                                                            jsize,
                                                                        len:
                                                                            jsize,
                                                                        buf:
                                                                            *mut jshort),
    pub GetIntArrayRegion: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv,
                                                                      array:
                                                                          jintArray,
                                                                      start:
                                                                          jsize,
                                                                      len:
                                                                          jsize,
                                                                      buf:
                                                                          *mut jint),
    pub GetLongArrayRegion: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       array:
                                                                           jlongArray,
                                                                       start:
                                                                           jsize,
                                                                       len:
                                                                           jsize,
                                                                       buf:
                                                                           *mut jlong),
    pub GetFloatArrayRegion: unsafe extern "C" fn(env:
                                                                            *mut JNIEnv,
                                                                        array:
                                                                            jfloatArray,
                                                                        start:
                                                                            jsize,
                                                                        len:
                                                                            jsize,
                                                                        buf:
                                                                            *mut jfloat),
    pub GetDoubleArrayRegion: unsafe extern "C" fn(env:
                                                                             *mut JNIEnv,
                                                                         array:
                                                                             jdoubleArray,
                                                                         start:
                                                                             jsize,
                                                                         len:
                                                                             jsize,
                                                                         buf:
                                                                             *mut jdouble),
    pub SetBooleanArrayRegion: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          array:
                                                                              jbooleanArray,
                                                                          start:
                                                                              jsize,
                                                                          l:
                                                                              jsize,
                                                                          buf:
                                                                              *const jboolean),
    pub SetByteArrayRegion: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       array:
                                                                           jbyteArray,
                                                                       start:
                                                                           jsize,
                                                                       len:
                                                                           jsize,
                                                                       buf:
                                                                           *const jbyte),
    pub SetCharArrayRegion: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       array:
                                                                           jcharArray,
                                                                       start:
                                                                           jsize,
                                                                       len:
                                                                           jsize,
                                                                       buf:
                                                                           *const jchar),
    pub SetShortArrayRegion: unsafe extern "C" fn(env:
                                                                            *mut JNIEnv,
                                                                        array:
                                                                            jshortArray,
                                                                        start:
                                                                            jsize,
                                                                        len:
                                                                            jsize,
                                                                        buf:
                                                                            *const jshort),
    pub SetIntArrayRegion: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv,
                                                                      array:
                                                                          jintArray,
                                                                      start:
                                                                          jsize,
                                                                      len:
                                                                          jsize,
                                                                      buf:
                                                                          *const jint),
    pub SetLongArrayRegion: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       array:
                                                                           jlongArray,
                                                                       start:
                                                                           jsize,
                                                                       len:
                                                                           jsize,
                                                                       buf:
                                                                           *const jlong),
    pub SetFloatArrayRegion: unsafe extern "C" fn(env:
                                                                            *mut JNIEnv,
                                                                        array:
                                                                            jfloatArray,
                                                                        start:
                                                                            jsize,
                                                                        len:
                                                                            jsize,
                                                                        buf:
                                                                            *const jfloat),
    pub SetDoubleArrayRegion: unsafe extern "C" fn(env:
                                                                             *mut JNIEnv,
                                                                         array:
                                                                             jdoubleArray,
                                                                         start:
                                                                             jsize,
                                                                         len:
                                                                             jsize,
                                                                         buf:
                                                                             *const jdouble),
    pub RegisterNatives: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    clazz:
                                                                        jclass,
                                                                    methods:
                                                                        *const JNINativeMethod,
                                                                    nMethods:
                                                                        jint)
                                                   -> jint,
    pub UnregisterNatives: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv,
                                                                      clazz:
                                                                          jclass)
                                                     -> jint,
    pub MonitorEnter: unsafe extern "C" fn(env:
                                                                     *mut JNIEnv,
                                                                 obj: jobject)
                                                -> jint,
    pub MonitorExit: unsafe extern "C" fn(env:
                                                                    *mut JNIEnv,
                                                                obj: jobject)
                                               -> jint,
    pub GetJavaVM: unsafe extern "C" fn(env:
                                                                  *mut JNIEnv,
                                                              vm:
                                                                  *mut *mut JavaVM)
                                             -> jint,
    pub GetStringRegion: unsafe extern "C" fn(env:
                                                                        *mut JNIEnv,
                                                                    str:
                                                                        jstring,
                                                                    start:
                                                                        jsize,
                                                                    len:
                                                                        jsize,
                                                                    buf:
                                                                        *mut jchar),
    pub GetStringUTFRegion: unsafe extern "C" fn(env:
                                                                           *mut JNIEnv,
                                                                       str:
                                                                           jstring,
                                                                       start:
                                                                           jsize,
                                                                       len:
                                                                           jsize,
                                                                       buf:
                                                                           *mut libc::c_char),
    pub GetPrimitiveArrayCritical: unsafe extern "C" fn(env:
                                                                                  *mut JNIEnv,
                                                                              array:
                                                                                  jarray,
                                                                              isCopy:
                                                                                  *mut jboolean)
                                                             ->
                                                                 *mut libc::c_void,
    pub ReleasePrimitiveArrayCritical: unsafe extern "C" fn(env:
                                                                                      *mut JNIEnv,
                                                                                  array:
                                                                                      jarray,
                                                                                  carray:
                                                                                      *mut libc::c_void,
                                                                                  mode:
                                                                                      jint),
    pub GetStringCritical: unsafe extern "C" fn(env:
                                                                          *mut JNIEnv,
                                                                      string:
                                                                          jstring,
                                                                      isCopy:
                                                                          *mut jboolean)
                                                     -> *const jchar,
    pub ReleaseStringCritical: unsafe extern "C" fn(env:
                                                                              *mut JNIEnv,
                                                                          string:
                                                                              jstring,
                                                                          cstring:
                                                                              *const jchar),
    pub NewWeakGlobalRef: unsafe extern "C" fn(env:
                                                                         *mut JNIEnv,
                                                                     obj:
                                                                         jobject)
                                                    -> jweak,
    pub DeleteWeakGlobalRef: unsafe extern "C" fn(env:
                                                                            *mut JNIEnv,
                                                                        _ref:
                                                                            jweak),
    pub ExceptionCheck: unsafe extern "C" fn(env:
                                                                       *mut JNIEnv)
                                                  -> jboolean,
    pub NewDirectByteBuffer: unsafe extern "C" fn(env:
                                                                            *mut JNIEnv,
                                                                        address:
                                                                            *mut libc::c_void,
                                                                        capacity:
                                                                            jlong)
                                                       -> jobject,
    pub GetDirectBufferAddress: unsafe extern "C" fn(env:
                                                                               *mut JNIEnv,
                                                                           buf:
                                                                               jobject)
                                                          ->
                                                              *mut libc::c_void,
    pub GetDirectBufferCapacity: unsafe extern "C" fn(env:
                                                                                *mut JNIEnv,
                                                                            buf:
                                                                                jobject)
                                                           -> jlong,
    pub GetObjectRefType: unsafe extern "C" fn(env:
                                                                         *mut JNIEnv,
                                                                     obj:
                                                                         jobject)
                                                    -> jobjectRefType,
}

impl Clone for JNINativeInterface_ {
    fn clone(&self) -> Self { *self }
}

impl Default for JNINativeInterface_ {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct JNIEnv_ {
    pub functions: *const JNINativeInterface_,
}

impl Clone for JNIEnv_ {
    fn clone(&self) -> Self { *self }
}

impl Default for JNIEnv_ {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct JavaVMOption {
    pub optionString: *mut libc::c_char,
    pub extraInfo: *mut libc::c_void,
}

impl Clone for JavaVMOption {
    fn clone(&self) -> Self { *self }
}

impl Default for JavaVMOption {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct JavaVMInitArgs {
    pub version: jint,
    pub nOptions: jint,
    pub options: *mut JavaVMOption,
    pub ignoreUnrecognized: jboolean,
}

impl Clone for JavaVMInitArgs {
    fn clone(&self) -> Self { *self }
}

impl Default for JavaVMInitArgs {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct JavaVMAttachArgs {
    pub version: jint,
    pub name: *mut libc::c_char,
    pub group: jobject,
}

impl Clone for JavaVMAttachArgs {
    fn clone(&self) -> Self { *self }
}

impl Default for JavaVMAttachArgs {
    fn default() -> Self { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct JNIInvokeInterface_ {
    pub reserved0: *mut libc::c_void,
    pub reserved1: *mut libc::c_void,
    pub reserved2: *mut libc::c_void,
    pub DestroyJavaVM: unsafe extern "C" fn(vm:
                                                                      *mut JavaVM)
                                                 -> jint,
    pub AttachCurrentThread: unsafe extern "C" fn(vm:
                                                                            *mut JavaVM,
                                                                        penv:
                                                                            *mut *mut libc::c_void,
                                                                        args:
                                                                            *mut libc::c_void)
                                                       -> jint,
    pub DetachCurrentThread: unsafe extern "C" fn(vm:
                                                                            *mut JavaVM)
                                                       -> jint,
    pub GetEnv: unsafe extern "C" fn(vm: *mut JavaVM,
                                                           penv:
                                                               *mut *mut libc::c_void,
                                                           version: jint)
                                          -> jint,
    pub AttachCurrentThreadAsDaemon: unsafe extern "C" fn(vm:
                                                                                    *mut JavaVM,
                                                                                penv:
                                                                                    *mut *mut libc::c_void,
                                                                                args:
                                                                                    *mut libc::c_void)
                                                               -> jint,
}

impl Clone for JNIInvokeInterface_ {
    fn clone(&self) -> Self { *self }
}

impl Default for JNIInvokeInterface_ {
    fn default() -> Self { unsafe { mem::zeroed() } }
}
