module Errata exposing (..)

import Html exposing (..)
import Html.Attributes exposing (href, target)


stateMachines =
    div []
<<<<<<< HEAD
        [ h2 [] [ text "Information" ]
        , h3 [] [ text "Authors" ]
        , p [] [ text "Proudly made by Neil Ashford, Simon Gordon, Callum Hays and Damian Van Kranendonk." ]
        , h3 [] [ text "About this application" ]
        , div []
            [ p [] [ text "State Machina is an automated tool to assist in the design of finite state machines." ]
            , p []
                [ text
                    ("The green state is the starting state of your machine. It cannot be deleted. "
                        ++ "Red states are the valid accepting / final states of your machine. "
                        ++ "The blue states are the other (non-accepting) states. "
                    )
                ]
            , p []
                [ text
                    ("Transitions between states are represented by arrows. The label \"input 1/input 2\" on "
                        ++ "a transition means that that transition can take place on input 1 or input 2."
                    )
                ]
            ]
        , h3 [] [ text "Resources" ]
        , ul []
            [ li []
                [ a
                    [ href "http://blog.markshead.com/869/state-machines-computer-science/"
                    , target "_blank"
                    ]
                    [ text "State Machines - Basics of Comuter Science"
                    ]
                ]
            , li []
                [ a
                    [ href "https://github.com/ashfordneil/state-machina/"
                    , target "_blank"
                    ]
                    [ text "Source code of this application"
                    ]
=======
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
>>>>>>> 289b35df32f3000674ab172a6ab6b2f0ca183792
                ]
                [ text "State Machines - Basics of Computer Science (Blog)" ]
            ]
        ]
