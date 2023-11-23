-injars toolkit/jre/lib/rt.jar
-outjars toolkit/runtime.jar

# Keep - only keep the classes from the package java.io
# and specifically PrintStream and its dependencies
-keep class java.io.PrintStream {
    *;
}

# Keep - classes that PrintStream depends on
-keep class java.io.FilterOutputStream {
    *;
}
-keep class java.io.OutputStream {
    *;
}

# Allow optimization, obfuscation, and preverification to be performed.
-optimizationpasses 5
-dontusemixedcaseclassnames
-dontskipnonpubliclibraryclasses
-dontpreverify
-dontobfuscate
-verbose

# Optimization options to remove unused code
-assumenosideeffects class java.io.PrintStream {
    public static void println(...);
}

# Don't warn about unresolved references and other issues,
# as we're intentionally stripping down the JAR file
-dontwarn
-ignorewarnings
