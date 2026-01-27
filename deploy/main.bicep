targetScope='resourceGroup'

module static_web_app 'br/public:avm/res/web/static-site:0.9.3' = {
    name: 'solver'
    params: {
        name: 'swa-solver'
        location: 'westeurope'
        sku: 'Free'
        customDomains: [
            {
                domainName: 'cds.newman.digital'
                validationMethod: 'CNAME'
            }
        ]
    }
}

@description('Output the default hostname')
output endpoint string = static_web_app.outputs.defaultHostname

@description('Output the static web app name')
output staticWebAppName string = static_web_app.outputs.name
