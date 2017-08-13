module Errata exposing (..)

import Html exposing (text, div, p, a, ul, li, h2, h3)
import Html.Attributes exposing (href, target)

stateMachines =
    div []
    [ 
        h2 [] [text "Information"],
        h3 [] [text "About this application"],
        p []
        [ text "This app allows users to build and optimise finite state machines." ],
        h3 [] [text "Group Members"],
        ul []
        [
            li [] [text "Neil Ashford (UQ)"],
            li [] [text "Damian Van Kranendonk (UQ)"],
            li [] [text "Callum Hays (QUT)"],
            li [] [text "Simon Gordon (QUT)"]
        ],
        h3 [] [text "Resources"],
        ul []
        [
            li []
            [
                a
                [
                    href "https://en.wikipedia.org/wiki/Finite-state_machine",
                    target "_blank"
                ]
                [ text "Finite-state machine (Wikipedia)" ]
            ],
            li []
            [
                a
                [
                    href "http://blog.markshead.com/869/state-machines-computer-science/",
                    target "_blank"
                ]
                [ text "State Machines - Basics of Computer Science (Blog)" ]
            ]
        ]
    ]
