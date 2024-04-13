grpcurl -plaintext '[::1]:50051' list

grpcurl -plaintext -proto ./proto/calculator.proto -d '{"a":2,"b":3}' [::1]:50051 calculator.Calculator.Add

grpcui -plaintext '[::1]:50051' 