@setlocal && pushd "%~dp0.."

cargo c %1

@endlocal && popd
