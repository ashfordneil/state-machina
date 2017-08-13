import { Dataset, Network } from "vis/index-network";

const registerEvents = (ports, network) => {
    network.on("selectEdge", ({ edges }) => {
        ports.edgeSelected.send(JSON.parse(edges[0]));
    });
};

module.exports = ports => {
    const networkMap = {};

    ports.initCmdPort.subscribe(({ divId, data, options }) => {
        for (var i = 0; i < data.edges.length; ++i) {
            const { from, to } = data.edges[i];
            data.edges[i].id = JSON.stringify({ from, to });
        }
        networkMap[divId] = new Network(
            document.getElementById(divId),
            data,
            options
        );

        registerEvents(ports, networkMap[divId]);
        ports.initSuccessfulPort.send(true);
    });

    ports.updateData.subscribe(([divId, data]) => {
        console.log(data);
        for (var i = 0; i < data.edges.length; ++i) {
            const { from, to } = data.edges[i];
            data.edges[i].id = JSON.stringify({ from, to });
        }
        networkMap[divId].setData(data);
    });
};
