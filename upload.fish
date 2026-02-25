#!/opt/homebrew/bin/fish

function upload
    rm -r target
    pack .
    scp -P 95 ./awb.tar.gz hawley@remote.deaddogsdaily.org:~/n8n/awb.tar.gz
end

upload
