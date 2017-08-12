import { Dataset, Network } from 'vis/index-network';

module.exports = ports => {
    const networkMap = {}

    ports.initCmdPort.subscribe(({ divId, data, options }) => {
        networkMap[divId] = new Network(
            document.getElementById(divId),
            data,
            options
        )

        ports.initSuccessfulPort.send(true)
    })
}