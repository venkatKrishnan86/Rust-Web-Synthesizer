# ASE-Project

Run `trunk serve --open`

## Synthesizer 
Topic Ideas: midi, synths/ MPE, UI, web dev 

### Motivation/ Problem to be solved/ Why is there a need for this 

- Physical synthesizers are heavy and take up a lot of space 
- Virtual synthesizers are more portable (digitally) and can be more cost effective 
- Browser based applications are more easily accessible 
- Synths user interfaces can sometime be cluttered and intimidating for novice users. By simplifying the UI, our synthesizer can be more welcoming and understandable.  
### Applications/ Use Cases/ Target Users/ Context/ Environment 
- Web application of a synthesizer for delivery to users interested in music tech software 
- The only user entry requirement would be familiarity with their computer setup 


### Functionality from User Point of View and How It Differentiates from Similar Products 

- Users will have the ability to use oscillators and different filters and EQs to generate sounds of users’ own design in their web browser 

- Users will also be able to use computer keyboard keys mapped to MIDI notes to change pitches of the oscillator output to compose and jam 

- Since the user interface is in the web browser and not in a DAW, our platform is novice friendly and easy to set up 

### Plans for implementation: flow chart, processing blocks, needed components, potential need for 3rd party libs 

#### Potential 3rd party lib 

- MIDI I/O 
- Touchpad Interface  

 

#### Flowchart  

1. Document all the required features of a synthesizer 
2. Plan the implementation of major functions:  <br>
    a. Audio Ring Buffer <br>
    b. MIDI Map Keyboard (MIDI using QWERTY keyboard) <br>
    c. Oscillators <br>
    d. Filters <br>
    e. LFO <br>
    f. Envelopes <br>
    g. XY Map for effects 

3. Create abstract classes and abstract functions 
4. Write unit tests for each functions’ utility 
5. Write integration tests 
6. Perform manual verification and testing to check performance 

#### Input: QWERTY keyboard -> MIDI keyboard 

- Touchpad Position Effect 
- Multi-Touch for Separate Effects 

#### Interface and output: a web UI for a digital synthesizer  

#### Four main components of the Synthesizer - 

1. Oscillators <br>
    a. Sine <br>
    b. Square <br>
    c. Triangle <br>
    d. White noise 

2. Filter Algorithms <br>
    a. Band Pass <br>
    b. Low Pass <br>
    c. High Pass 

3. LFO Algorithms <br>
    a. Sine LFO  <br>
    b. Square LFO  <br>
    c. Bidirectional Square LFO  <br> 
    d. Triangle LFO 

4. Envelope Algorithms  <br>
    a. ADSR <br>
    b. AHDSR (If there is time) 

Linking LFOs and Envelopes to Oscillators and Filters 

### Algorithmic references - which reference do you base your algorithmic implementations on? 

- Needs background research? 
- Roli as reference for XY effects mapping controller 

 

### General responsibilities and work assignments (can overlap) 

- Keyboard IO and mapping backend (Yiwei) 
    - Input reading 
    - Mapping (TeAiris) 

- Audio synth backend (Venkat + support) 
    - Refer to “Four main components” section 
    - Algorithm implementation (Efficient) 
    - Integration of individual components 

- Web and UI interface/frontend (Nicolette + Aleksandra – web dev) 
  - UI design (If there is time) (TeAiris)  
  - Web implementation (Yew) 
    - https://github.com/yewstack/yew 
    - https://yew.rs/ 

- Testing  
  - Unit tests (Everyone) 
  - Integration tests  
    - E.g. envelopes and filters 

- Documentation 
  - Document all the required features of a synthesizer 
    - Requirements to satisfy for future testing 
  - Within Rust – Documenting the code using `cargo doc` 

 
 
# ASE-Project
 
