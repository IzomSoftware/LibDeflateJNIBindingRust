cd rust/ && cargo b -r && cd ../
rm java/src/main/resources/linux-x86-64/libdeflater_jni_binding.so
cp rust/target/release/libdeflater_jni_binding.so java/src/main/resources/linux-x86-64/
cd java/ && mvn clean install