# Speakeasy API Smoke Test
$baseUrl = "http://localhost:3000"

function Test-Endpoint($name, $path, $method = "GET", $body = $null) {
    Write-Host "Testing $name ($path)..." -ForegroundColor Cyan
    try {
        $params = @{
            Uri = "$baseUrl$path"
            Method = $method
            ContentType = "application/json"
        }
        if ($body) { $params.Body = ($body | ConvertTo-Json) }
        
        $resp = Invoke-RestMethod @params
        Write-Host "Success!" -ForegroundColor Green
        return $resp
    } catch {
        Write-Host "Failed: $($_.Exception.Message)" -ForegroundColor Red
        return $null
    }
}

# 1. Health Check
Test-Endpoint "Health Check" "/health"

# 2. Register
$reg = Test-Endpoint "Register" "/v1/auth/register" "POST" @{
    display_name = "Test User"
}

if ($reg) {
    $token = $reg.access_token
    $userId = $reg.user_id
    Write-Host "Auth Token: $token"
    
    # Header for authenticated calls
    $headers = @{ Authorization = "Bearer $token" }

    # 3. Keys Upload
    Test-Endpoint "Keys Upload" "/v1/keys/upload" "POST" @{
        user_id = $userId
        device_id = [guid]::NewGuid().ToString()
        identity_key_ed25519_b64 = "base64_id_key"
        static_x25519_b64 = "base64_static_key"
        signed_prekey_x25519_b64 = "base64_signed_prekey"
        signed_prekey_signature_b64 = "base64_sig"
        one_time_prekeys_b64 = @("otk1")
    }
}
