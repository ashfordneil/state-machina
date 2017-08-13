import { Dataset, Network } from 'vis/index-network';

const registerEvents = (data, ports, network) => {
    network.on("selectEdge", ({edges}) => {
        ports.edgeSelected.send(data.edges.filter(edge => edge.id === edges[0])[0]);
    });
}

module.exports = ports => {
    const networkMap = {}

    ports.initCmdPort.subscribe(({ divId, data, options }) => {
        networkMap[divId] = new Network(
            document.getElementById(divId),
            data,
            options
        )

        registerEvents(data, ports, networkMap[divId]);
        ports.initSuccessfulPort.send(true)

    })

    ports.updateData.subscribe(([divId, data]) => {
        networkMap[divId].setData(data)
    })
}