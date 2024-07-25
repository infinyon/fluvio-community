fluvio cloud connector delete helsinki-bus
sleep 5
fluvio topic delete helsinki
fluvio sm delete infinyon/jolt@0.1.0 
fluvio sm delete infinyon/regex-filter@0.1.0
fluvio sm delete group1/stat@0.1.0
cp regex.yaml t.yaml
