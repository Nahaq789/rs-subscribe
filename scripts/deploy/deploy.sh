echo "Deploy to AWS"
echo "Running terraform init..."
terraform init

if [ $? -eq 0 ]; then
    echo "terraform init success"
    echo "Running terraform plan..."
    terraform plan

    if [ $? -eq 0 ]; then
        echo "terraform plan success"
        echo "Running terraform apply..."
        terraform apply -auto-approve

        if [ $? -eq 0 ]; then
            echo "terraform apply success"
        else
            echo "Failed: terraform apply"
            exit 1
        fi
    else
        echo "Failed: terraform plan"
        exit 1
    fi
else
    echo "Failed: terraform init"
    exit 1
fi